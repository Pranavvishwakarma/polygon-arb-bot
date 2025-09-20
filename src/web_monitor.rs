use axum::{
    routing::get,
    Router,
    Json,
    response::Html,
};
use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ArbData {
    pub timestamp: u64,
    pub direction: String,
    pub profit_usdc: f64,
}

pub type SharedArbData = Arc<Mutex<Vec<ArbData>>>;

pub async fn get_arb_data(state: SharedArbData) -> Json<Vec<ArbData>> {
    let data = state.lock().unwrap();
    Json(data.clone())
}

pub async fn dashboard() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Arbitrage Bot Monitor Enterprise</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="Professional Arbitrage Trading Monitor Dashboard">
    
    <!-- Preload critical resources -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    
    <style>
        /* CSS Variables for theming */
        :root {
            --primary-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            --secondary-gradient: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            --success-gradient: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            --dark-bg: #0f0f23;
            --card-bg: rgba(255, 255, 255, 0.98);
            --text-primary: #2c3e50;
            --text-secondary: #7f8c8d;
            --success-color: #27ae60;
            --warning-color: #f39c12;
            --error-color: #e74c3c;
            --border-radius: 16px;
            --transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            --shadow-sm: 0 2px 8px rgba(0, 0, 0, 0.1);
            --shadow-md: 0 4px 20px rgba(0, 0, 0, 0.15);
            --shadow-lg: 0 8px 32px rgba(0, 0, 0, 0.2);
        }

        /* Dark mode support */
        @media (prefers-color-scheme: dark) {
            :root {
                --card-bg: rgba(26, 26, 46, 0.98);
                --text-primary: #ffffff;
                --text-secondary: #a0a0b8;
            }
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: var(--dark-bg);
            background-image: 
                radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%),
                radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%),
                radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.2) 0%, transparent 50%);
            min-height: 100vh;
            color: var(--text-primary);
            line-height: 1.6;
            overflow-x: hidden;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }

        /* Header Section */
        .header {
            text-align: center;
            margin-bottom: 40px;
            position: relative;
        }

        .header::before {
            content: '';
            position: absolute;
            top: -50%;
            left: 50%;
            transform: translateX(-50%);
            width: 200px;
            height: 200px;
            background: var(--primary-gradient);
            border-radius: 50%;
            filter: blur(100px);
            opacity: 0.3;
            z-index: -1;
        }

        .header h1 {
            font-size: 3rem;
            font-weight: 700;
            background: var(--primary-gradient);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 10px;
            animation: fadeInUp 0.8s ease-out;
        }

        .header p {
            font-size: 1.2rem;
            color: var(--text-secondary);
            animation: fadeInUp 0.8s ease-out 0.2s both;
        }

        /* Status Bar */
        .status-bar {
            display: flex;
            justify-content: center;
            align-items: center;
            gap: 12px;
            margin-bottom: 30px;
            animation: fadeInUp 0.8s ease-out 0.4s both;
        }

        .status-indicator {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color:white;
            padding: 12px 24px;
            background: rgba(39, 174, 96, 0.1);
            border: 1px solid rgba(39, 174, 96, 0.2);
            border-radius: 50px;
            backdrop-filter: blur(10px);
        }

        .status-dot {
            width: 10px;
            height: 10px;
            border-radius: 50%;
            background: var(--success-color);
            animation: pulse 2s infinite;
            box-shadow: 0 0 0 0 rgba(39, 174, 96, 0.7);
        }

        @keyframes pulse {
            0% { 
                transform: scale(0.95);
                box-shadow: 0 0 0 0 rgba(39, 174, 96, 0.7);
            }
            70% { 
                transform: scale(1);
                box-shadow: 0 0 0 10px rgba(39, 174, 96, 0);
            }
            100% { 
                transform: scale(0.95);
                box-shadow: 0 0 0 0 rgba(39, 174, 96, 0);
            }
        }

        /* Stats Grid */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 24px;
            margin-bottom: 40px;
        }

        .stat-card {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 32px;
            box-shadow: var(--shadow-md);
            transition: var(--transition);
            position: relative;
            overflow: hidden;
            animation: fadeInUp 0.8s ease-out;
            animation-fill-mode: both;
        }

        .stat-card:nth-child(1) { animation-delay: 0.1s; }
        .stat-card:nth-child(2) { animation-delay: 0.2s; }
        .stat-card:nth-child(3) { animation-delay: 0.3s; }
        .stat-card:nth-child(4) { animation-delay: 0.4s; }

        .stat-card:hover {
            transform: translateY(-8px);
            box-shadow: var(--shadow-lg);
        }

        .stat-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 4px;
            background: var(--primary-gradient);
        }

        .stat-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
        }

        .stat-icon {
            width: 48px;
            height: 48px;
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            background: var(--primary-gradient);
            color: white;
        }

        .stat-icon svg {
            width: 24px;
            height: 24px;
            fill: currentColor;
        }

        .stat-title {
            font-size: 0.9rem;
            font-weight: 500;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .stat-value {
            font-size: 2.5rem;
            font-weight: 700;
            color: var(--text-primary);
            margin-bottom: 8px;
        }

        .stat-change {
            font-size: 0.9rem;
            font-weight: 500;
            display: flex;
            align-items: center;
            gap: 4px;
        }

        .stat-change.positive {
            color: var(--success-color);
        }

        .stat-change.negative {
            color: var(--error-color);
        }

        .change-icon {
            width: 16px;
            height: 16px;
        }

        /* Main Content Grid */
        .content-grid {
            display: grid;
            grid-template-columns: 1fr 400px;
            gap: 24px;
            margin-bottom: 24px;
        }

        @media (max-width: 1024px) {
            .content-grid {
                grid-template-columns: 1fr;
            }
        }

        /* Chart Container */
        .chart-container {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 32px;
            box-shadow: var(--shadow-md);
            animation: fadeInUp 0.8s ease-out 0.5s both;
        }

        .chart-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 24px;
        }

        .chart-title {
            font-size: 1.25rem;
            font-weight: 600;
            color: var(--text-primary);
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .chart-icon {
            width: 20px;
            height: 20px;
            fill: currentColor;
        }

        .chart-canvas {
            width: 100%;
            height: 300px;
            background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: var(--text-secondary);
            font-size: 1.1rem;
        }

        /* Arbitrage List */
        .arbitrage-container {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 32px;
            box-shadow: var(--shadow-md);
            animation: fadeInUp 0.8s ease-out 0.6s both;
        }

        .arbitrage-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 24px;
        }

        .arbitrage-title {
            font-size: 1.25rem;
            font-weight: 600;
            color: var(--text-primary);
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .target-icon {
            width: 20px;
            height: 20px;
            fill: currentColor;
        }

        .arbitrage-count {
            background: var(--primary-gradient);
            color: white;
            padding: 6px 12px;
            border-radius: 20px;
            font-size: 0.9rem;
            font-weight: 500;
        }

        .arbitrage-list {
            max-height: 400px;
            overflow-y: auto;
            padding-right: 8px;
        }

        .arbitrage-list::-webkit-scrollbar {
            width: 6px;
        }

        .arbitrage-list::-webkit-scrollbar-track {
            background: rgba(0, 0, 0, 0.1);
            border-radius: 3px;
        }

        .arbitrage-list::-webkit-scrollbar-thumb {
            background: var(--primary-gradient);
            border-radius: 3px;
        }

        .arb-item {
            background: rgba(255, 255, 255, 0.05);
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 12px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            transition: var(--transition);
            position: relative;
            overflow: hidden;
        }

        .arb-item:hover {
            background: rgba(255, 255, 255, 0.08);
            transform: translateX(4px);
        }

        .arb-item::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            bottom: 0;
            width: 4px;
            background: var(--success-gradient);
        }

        .arb-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 12px;
        }

        .arb-direction {
            font-weight: 600;
            font-size: 1.1rem;
            color: var(--text-primary);
        }

        .arb-profit {
            font-size: 1.4rem;
            font-weight: 700;
            color: var(--success-color);
            display: flex;
            align-items: center;
            gap: 4px;
        }

        .arb-details {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 0.9rem;
            color: var(--text-secondary);
        }

        .arb-timestamp {
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .arb-badge {
            background: rgba(39, 174, 96, 0.2);
            color: var(--success-color);
            padding: 4px 8px;
            border-radius: 6px;
            font-size: 0.8rem;
            font-weight: 500;
        }

        /* Loading State */
        .loading-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 60px;
            color: var(--text-secondary);
        }

        .loading-spinner {
            width: 48px;
            height: 48px;
            border: 4px solid rgba(255, 255, 255, 0.1);
            border-left: 4px solid var(--success-color);
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-bottom: 16px;
        }

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        /* Error State */
        .error-container {
            text-align: center;
            padding: 60px;
            color: var(--error-color);
        }

        .error-icon {
            font-size: 48px;
            margin-bottom: 16px;
        }

        /* Refresh Indicator */
        .refresh-indicator {
            position: fixed;
            top: 24px;
            right: 24px;
            background: var(--card-bg);
            padding: 12px 20px;
            border-radius: 50px;
            box-shadow: var(--shadow-md);
            backdrop-filter: blur(10px);
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.9rem;
            color: var(--text-secondary);
            animation: fadeInRight 0.8s ease-out 0.8s both;
            z-index: 1000;
        }

        .refresh-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: var(--success-color);
            animation: pulse 2s infinite;
        }

        /* Animations */
        @keyframes fadeInUp {
            from {
                opacity: 0;
                transform: translateY(30px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        @keyframes fadeInRight {
            from {
                opacity: 0;
                transform: translateX(30px);
            }
            to {
                opacity: 1;
                transform: translateX(0);
            }
        }

        /* Responsive Design */
        @media (max-width: 768px) {
            .header h1 {
                font-size: 2rem;
            }
            
            .stats-grid {
                grid-template-columns: 1fr;
            }
            
            .stat-card {
                padding: 24px;
            }
            
            .container {
                padding: 16px;
            }
        }

        /* Performance optimizations */
        .arb-item {
            will-change: transform;
            contain: layout style;
        }

        .stat-card {
            will-change: transform;
            contain: layout style;
        }
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1>Arbitrage Monitor Pro</h1>
            <p>Real-time arbitrage opportunity tracking and analytics</p>
        </header>

        <div class="status-bar">
            <div class="status-indicator">
                <div class="status-dot"></div>
                <span id="status">Initializing...</span>
            </div>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">📊</div>
                </div>
                <div class="stat-value" id="total-opportunities">0</div>
                <div class="stat-title">Total Opportunities</div>
                <div class="stat-change positive">
                    <span>↗</span>
                    <span id="opportunities-change">+0%</span>
                </div>
            </div>

            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">💰</div>
                </div>
                <div class="stat-value" id="total-profit">0.00</div>
                <div class="stat-title">Total Profit (USDC)</div>
                <div class="stat-change positive">
                    <span>↗</span>
                    <span id="profit-change">+0%</span>
                </div>
            </div>

            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">📈</div>
                </div>
                <div class="stat-value" id="avg-profit">0.00</div>
                <div class="stat-title">Average Profit</div>
                <div class="stat-change positive">
                    <span>↗</span>
                    <span id="avg-change">+0%</span>
                </div>
            </div>

            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">⚡</div>
                </div>
                <div class="stat-value" id="success-rate">0%</div>
                <div class="stat-title">Success Rate</div>
                <div class="stat-change positive">
                    <span>↗</span>
                    <span id="rate-change">+0%</span>
                </div>
            </div>
        </div>

        <div class="content-grid">
            <div class="chart-container">
                <div class="chart-header">
                    <h3 class="chart-title">📊 Profit Trend Analysis</h3>
                    <div class="arbitrage-count" id="chart-timeframe">Last 24h</div>
                </div>
                <div class="chart-canvas" id="profit-chart">
                    <div>Real-time chart will be rendered here</div>
                </div>
            </div>

            <div class="arbitrage-container">
                <div class="arbitrage-header">
                    <h3 class="arbitrage-title">🎯 Live Opportunities</h3>
                    <div class="arbitrage-count" id="live-count">0 Active</div>
                </div>
                <div class="arbitrage-list" id="arbitrage-list">
                    <div class="loading-container">
                        <div class="loading-spinner"></div>
                        <div>Loading arbitrage opportunities...</div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="refresh-indicator">
        <div class="refresh-dot"></div>
        <span>Last update: <span id="last-update">--</span></span>
    </div>

    <script>
        class ArbitrageMonitor {
            constructor() {
                this.data = [];
                this.previousData = [];
                this.updateInterval = 5000;
                this.maxRetries = 3;
                this.retryCount = 0;
                this.isConnected = false;
                
                this.init();
            }

            async init() {
                await this.fetchData();
                this.startAutoUpdate();
                this.updateLastUpdateTime();
            }

            async fetchData() {
                try {
                    const response = await fetch('/api/arb');
                    if (!response.ok) throw new Error(`HTTP ${response.status}`);
                    
                    const newData = await response.json();
                    this.previousData = [...this.data];
                    this.data = newData;
                    this.isConnected = true;
                    this.retryCount = 0;
                    
                    this.updateUI();
                    this.updateStatus('Connected', 'connected');
                } catch (error) {
                    console.error('Failed to fetch data:', error);
                    this.retryCount++;
                    
                    if (this.retryCount >= this.maxRetries) {
                        this.updateStatus('Connection Failed', 'error');
                        this.isConnected = false;
                    } else {
                        this.updateStatus(`Retrying... (${this.retryCount}/${this.maxRetries})`, 'warning');
                    }
                }
            }

            updateUI() {
                this.updateStats();
                this.updateArbitrageList();
                this.updateLastUpdateTime();
            }

            updateStats() {
                const totalOpportunities = this.data.length;
                const totalProfit = this.data.reduce((sum, item) => sum + item.profit_usdc, 0);
                const avgProfit = totalOpportunities > 0 ? totalProfit / totalOpportunities : 0;
                const successRate = totalOpportunities > 0 ? 100 : 0; // Simplified success rate

                // Calculate changes from previous data
                const prevTotal = this.previousData.length;
                const prevTotalProfit = this.previousData.reduce((sum, item) => sum + item.profit_usdc, 0);
                
                const opportunitiesChange = prevTotal > 0 ? ((totalOpportunities - prevTotal) / prevTotal * 100).toFixed(1) : 0;
                const profitChange = prevTotalProfit > 0 ? ((totalProfit - prevTotalProfit) / prevTotalProfit * 100).toFixed(1) : 0;

                // Update DOM
                document.getElementById('total-opportunities').textContent = totalOpportunities.toLocaleString();
                document.getElementById('total-profit').textContent = totalProfit.toFixed(2);
                document.getElementById('avg-profit').textContent = avgProfit.toFixed(2);
                document.getElementById('success-rate').textContent = successRate + '%';
                
                document.getElementById('opportunities-change').textContent = (opportunitiesChange >= 0 ? '+' : '') + opportunitiesChange + '%';
                document.getElementById('profit-change').textContent = (profitChange >= 0 ? '+' : '') + profitChange + '%';
                document.getElementById('avg-change').textContent = '+0%'; // Placeholder
                document.getElementById('rate-change').textContent = '+0%'; // Placeholder
            }

            updateArbitrageList() {
                const listContainer = document.getElementById('arbitrage-list');
                const liveCount = document.getElementById('live-count');
                
                if (!this.isConnected) {
                    listContainer.innerHTML = `
                        <div class="error-container">
                            <div class="error-icon">❌</div>
                            <div>Connection failed. Retrying...</div>
                        </div>
                    `;
                    liveCount.textContent = '0 Active';
                    return;
                }

                if (this.data.length === 0) {
                    listContainer.innerHTML = `
                        <div class="loading-container">
                            <div style="font-size: 48px; margin-bottom: 16px;">🔍</div>
                            <div>No arbitrage opportunities found</div>
                            <div style="font-size: 0.9rem; margin-top: 8px;">Monitoring markets for profitable trades...</div>
                        </div>
                    `;
                    liveCount.textContent = '0 Active';
                    return;
                }

                // Sort by profit (descending) and take top 10
                const recentData = [...this.data]
                    .sort((a, b) => b.profit_usdc - a.profit_usdc)
                    .slice(0, 10);

                listContainer.innerHTML = recentData.map((item, index) => `
                    <div class="arb-item" style="animation-delay: ${index * 0.1}s">
                        <div class="arb-header">
                            <div class="arb-direction">${this.escapeHtml(item.direction)}</div>
                            <div class="arb-profit">
                                <span>+$${item.profit_usdc.toFixed(2)}</span>
                                <span class="arb-badge">USDC</span>
                            </div>
                        </div>
                        <div class="arb-details">
                            <div class="arb-timestamp">
                                <span>🕐</span>
                                <span>${new Date(item.timestamp * 1000).toLocaleString()}</span>
                            </div>
                            <div class="arb-confidence">
                                <span style="color: var(--success-color);">●</span>
                                <span>High Confidence</span>
                            </div>
                        </div>
                    </div>
                `).join('');

                liveCount.textContent = `${this.data.length} Active`;
            }

            updateStatus(text, type) {
                const statusElement = document.getElementById('status');
                const statusDot = document.querySelector('.status-dot');
                
                statusElement.textContent = text;
                
                // Update status dot color
                const colors = {
                    connected: '#27ae60',
                    warning: '#f39c12',
                    error: '#e74c3c'
                };
                
                statusDot.style.background = colors[type] || colors.connected;
            }

            updateLastUpdateTime() {
                const now = new Date();
                document.getElementById('last-update').textContent = now.toLocaleTimeString();
            }

            escapeHtml(text) {
                const div = document.createElement('div');
                div.textContent = text;
                return div.innerHTML;
            }

            startAutoUpdate() {
                setInterval(() => {
                    this.fetchData();
                }, this.updateInterval);
            }
        }

        // Initialize the monitor when DOM is loaded
        document.addEventListener('DOMContentLoaded', () => {
            new ArbitrageMonitor();
        });

        // Add some interactive effects
        document.addEventListener('DOMContentLoaded', () => {
            // Add hover effects to stat cards
            const statCards = document.querySelectorAll('.stat-card');
            statCards.forEach(card => {
                card.addEventListener('mouseenter', function() {
                    this.style.transform = 'translateY(-8px) scale(1.02)';
                });
                
                card.addEventListener('mouseleave', function() {
                    this.style.transform = 'translateY(0) scale(1)';
                });
            });

            // Add click effects to arbitrage items
            const arbitrageList = document.getElementById('arbitrage-list');
            arbitrageList.addEventListener('click', function(e) {
                const item = e.target.closest('.arb-item');
                if (item) {
                    item.style.transform = 'scale(0.98)';
                    setTimeout(() => {
                        item.style.transform = 'scale(1)';
                    }, 150);
                }
            });
        });
    </script>
</body>
</html>
    "#)
}

pub async fn start_server(state: SharedArbData) {
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/api/arb", get({
            let state = state.clone();
            move || get_arb_data(state.clone())
        }));

    println!("Web monitoring server starting on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
