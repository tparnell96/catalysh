use std::io::{self, Stdout};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table, TableState, Tabs},
};
use serde::Deserialize;

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::{command_utils::CommandContext, http};

#[derive(Clone, Copy, PartialEq)]
pub enum TimeRange {
    H3,
    H24,
    D7,
    D30,
}

impl TimeRange {
    fn label(&self) -> &'static str {
        match self {
            Self::H3 => "3h",
            Self::H24 => "24h",
            Self::D7 => "7d",
            Self::D30 => "30d",
        }
    }

    fn millis_ago(&self) -> i64 {
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        let offset = match self {
            Self::H3 => 3 * 60 * 60 * 1000,
            Self::H24 => 24 * 60 * 60 * 1000,
            Self::D7 => 7 * 24 * 60 * 60 * 1000,
            Self::D30 => 30 * 24 * 60 * 60 * 1000,
        };
        ms - offset
    }

    fn all() -> [Self; 4] {
        [Self::D30, Self::D7, Self::H24, Self::H3]
    }

    fn tab_index(&self) -> usize {
        match self {
            Self::D30 => 0,
            Self::D7 => 1,
            Self::H24 => 2,
            Self::H3 => 3,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::D30 => Self::D7,
            Self::D7 => Self::H24,
            Self::H24 => Self::H3,
            Self::H3 => Self::D30,
        }
    }
}

pub struct HealthApp {
    pub time_range: TimeRange,
    pub client_total: Option<ClientBand>,
    pub client_wired: Option<ClientBand>,
    pub client_wireless: Option<ClientBand>,
    pub sites: Vec<SiteHealthRow>,
    pub site_table_state: TableState,
    pub loading: bool,
    pub error: Option<String>,
}

impl HealthApp {
    fn new() -> Self {
        let mut site_table_state = TableState::default();
        site_table_state.select(None);
        Self {
            time_range: TimeRange::D30,
            client_total: None,
            client_wired: None,
            client_wireless: None,
            sites: Vec::new(),
            site_table_state,
            loading: false,
            error: None,
        }
    }

    fn selected_site(&self) -> Option<usize> {
        self.site_table_state.selected()
    }

    fn sync_site_selection(&mut self) {
        if self.sites.is_empty() {
            self.site_table_state.select(None);
            return;
        }

        let selected = self.selected_site().unwrap_or(0).min(self.sites.len() - 1);
        self.site_table_state.select(Some(selected));
    }

    fn next_site(&mut self) {
        if self.sites.is_empty() {
            self.site_table_state.select(None);
            return;
        }

        let next = match self.selected_site() {
            Some(i) if i + 1 < self.sites.len() => i + 1,
            _ => self.sites.len() - 1,
        };
        self.site_table_state.select(Some(next));
    }

    fn previous_site(&mut self) {
        if self.sites.is_empty() {
            self.site_table_state.select(None);
            return;
        }

        let prev = match self.selected_site() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.site_table_state.select(Some(prev));
    }
}

pub struct ClientBand {
    pub count: i64,
    pub health_pct: u16,
}

pub struct SiteHealthRow {
    pub name: String,
    pub site_type: String,
    pub health_pct: u16,
    pub client_count: i64,
    pub device_health_pct: u16,
}

#[derive(Deserialize)]
struct ClientHealthResp {
    response: Vec<ClientHealthSite>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientHealthSite {
    score_detail: Option<Vec<ScoreDetail>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScoreDetail {
    score_category: Option<ScoreCategory>,
    score_value: Option<i64>,
    client_count: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScoreCategory {
    value: Option<String>,
}

#[derive(Deserialize)]
struct SiteHealthResp {
    response: Option<Vec<SiteHealthItem>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SiteHealthItem {
    site_name: Option<String>,
    site_type: Option<String>,
    healthy_clients_percentage: Option<serde_json::Value>,
    number_of_clients: Option<i64>,
    healthy_network_device_percentage: Option<serde_json::Value>,
}

async fn fetch_data(app: &mut HealthApp, config: &Config, token: &Token) {
    app.loading = true;
    app.error = None;

    let result = async {
        let client = http::build_client(config)?;
        let timestamp = app.time_range.millis_ago();

        let client_url = format!(
            "{}/dna/intent/api/v1/client-health?timestamp={}",
            config.dnac_url, timestamp
        );
        let site_url = format!(
            "{}/dna/intent/api/v1/site-health?timestamp={}",
            config.dnac_url, timestamp
        );

        let client_resp: ClientHealthResp =
            http::get_authenticated(&client, config, token, &client_url).await?;
        let site_resp: SiteHealthResp =
            http::get_authenticated(&client, config, token, &site_url).await?;

        populate_client_bands(app, client_resp);
        populate_sites(app, site_resp);
        Ok::<(), anyhow::Error>(())
    }
    .await;

    if let Err(err) = result {
        app.error = Some(err.to_string());
        app.client_total = None;
        app.client_wired = None;
        app.client_wireless = None;
        app.sites.clear();
        app.site_table_state.select(None);
    }

    app.loading = false;
}

fn populate_client_bands(app: &mut HealthApp, response: ClientHealthResp) {
    app.client_total = None;
    app.client_wired = None;
    app.client_wireless = None;

    let details = response
        .response
        .into_iter()
        .next()
        .and_then(|site| site.score_detail)
        .unwrap_or_default();

    for detail in details {
        let band = ClientBand {
            count: detail.client_count.unwrap_or(0),
            health_pct: clamp_pct(detail.score_value),
        };

        match detail
            .score_category
            .and_then(|category| category.value)
            .unwrap_or_default()
            .as_str()
        {
            "TOTAL" => app.client_total = Some(band),
            "WIRED" => app.client_wired = Some(band),
            "WIRELESS" => app.client_wireless = Some(band),
            _ => {}
        }
    }
}

fn populate_sites(app: &mut HealthApp, response: SiteHealthResp) {
    app.sites = response
        .response
        .unwrap_or_default()
        .into_iter()
        .map(|site| SiteHealthRow {
            name: site.site_name.unwrap_or_else(|| "N/A".to_string()),
            site_type: site.site_type.unwrap_or_else(|| "N/A".to_string()),
            health_pct: value_to_pct(site.healthy_clients_percentage.as_ref()),
            client_count: site.number_of_clients.unwrap_or(0),
            device_health_pct: value_to_pct(site.healthy_network_device_percentage.as_ref()),
        })
        .collect();
    app.sync_site_selection();
}

fn clamp_pct(value: Option<i64>) -> u16 {
    value.unwrap_or(0).clamp(0, 100) as u16
}

fn value_to_pct(value: Option<&serde_json::Value>) -> u16 {
    let pct = match value {
        Some(serde_json::Value::Number(number)) => {
            if let Some(v) = number.as_u64() {
                v as i64
            } else if let Some(v) = number.as_i64() {
                v
            } else {
                number.as_f64().unwrap_or_default().round() as i64
            }
        }
        Some(serde_json::Value::String(text)) => {
            text.parse::<f64>().unwrap_or_default().round() as i64
        }
        _ => 0,
    };
    pct.clamp(0, 100) as u16
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut HealthApp,
    config: &Config,
    token: &Token,
) -> Result<()> {
    let runtime = tokio::runtime::Runtime::new().context("Failed to create Tokio runtime")?;
    runtime.block_on(fetch_data(app, config, token));

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if event::poll(Duration::from_millis(200))?
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                KeyCode::Char('r') => runtime.block_on(fetch_data(app, config, token)),
                KeyCode::Tab => {
                    app.time_range = app.time_range.next();
                    runtime.block_on(fetch_data(app, config, token));
                }
                KeyCode::Up => app.previous_site(),
                KeyCode::Down => app.next_site(),
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &mut HealthApp) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let title = Paragraph::new("Catalysh — Health Dashboard").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(title, areas[0]);

    let tabs = Tabs::new(
        TimeRange::all()
            .into_iter()
            .map(|range| Line::from(Span::raw(format!(" {} ", range.label()))))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL))
    .select(app.time_range.tab_index())
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(tabs, areas[1]);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(areas[2]);

    render_client_health(frame, main[0], app);
    render_site_health(frame, main[1], app);

    let status_text = if app.loading {
        "Loading…".to_string()
    } else if let Some(error) = &app.error {
        format!(
            "Error: {} | Tab: switch time range | ↑↓: scroll sites | r: refresh | q: quit",
            error
        )
    } else {
        "Tab: switch time range | ↑↓: scroll sites | r: refresh | q: quit".to_string()
    };
    frame.render_widget(
        Paragraph::new(status_text).style(Style::default().fg(Color::DarkGray)),
        areas[3],
    );
}

fn render_client_health(frame: &mut Frame, area: Rect, app: &HealthApp) {
    let block = Block::default()
        .title("Client Health")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    render_client_band(frame, rows[0], "Total clients", app.client_total.as_ref());
    render_client_band(frame, rows[1], "Wired clients", app.client_wired.as_ref());
    render_client_band(
        frame,
        rows[2],
        "Wireless clients",
        app.client_wireless.as_ref(),
    );
}

fn render_client_band(frame: &mut Frame, area: Rect, label: &str, band: Option<&ClientBand>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18),
            Constraint::Length(8),
            Constraint::Min(10),
        ])
        .split(area);

    let count = band.map(|value| value.count).unwrap_or(0);
    let pct = band.map(|value| value.health_pct).unwrap_or(0);

    frame.render_widget(Paragraph::new(label), chunks[0]);
    frame.render_widget(
        Paragraph::new(count.to_string()).style(Style::default().add_modifier(Modifier::BOLD)),
        chunks[1],
    );
    frame.render_widget(
        Gauge::default()
            .gauge_style(gauge_style(pct))
            .percent(pct)
            .label(format!("{pct}%")),
        chunks[2],
    );
}

fn render_site_health(frame: &mut Frame, area: Rect, app: &mut HealthApp) {
    let header = Row::new(vec![
        Cell::from("Name"),
        Cell::from("Type"),
        Cell::from("Health%"),
        Cell::from("Clients"),
        Cell::from("Devices"),
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows = app.sites.iter().map(|site| {
        Row::new(vec![
            Cell::from(site.name.clone()),
            Cell::from(site.site_type.clone()),
            Cell::from(site.health_pct.to_string()),
            Cell::from(site.client_count.to_string()),
            Cell::from(site.device_health_pct.to_string()),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(34),
            Constraint::Percentage(18),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
        ],
    )
    .header(header)
    .block(Block::default().title("Site Health").borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(table, area, &mut app.site_table_state);
}

fn gauge_style(pct: u16) -> Style {
    let color = match pct {
        80..=100 => Color::Green,
        50..=79 => Color::Yellow,
        _ => Color::Red,
    };
    Style::default().fg(color)
}

pub fn launch_health_tui() -> Result<()> {
    let runtime = tokio::runtime::Runtime::new().context("Failed to create Tokio runtime")?;
    let ctx = runtime.block_on(CommandContext::new())?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = HealthApp::new();
    let result = run_app(&mut terminal, &mut app, &ctx.config, &ctx.token);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
