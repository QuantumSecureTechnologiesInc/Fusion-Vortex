const mockEvents = [
    { type: 'ExecutionStart', component: 'runtime', title: 'System Reset', detail: 'Initialization sequence complete.', icon: '🔄', time: '5m ago' },
    { type: 'PolicyDecision', component: 'policy', title: 'Capability Gated', detail: 'Allowed capability.network.external based on trust score.', icon: '🛡️', time: '3m ago' },
    { type: 'TrustUpdate', component: 'observer', title: 'Trust Score Verified', detail: 'Current score: 0.854. Trending positive.', icon: '📈', time: '1m ago' },
    { type: 'ToolCall', component: 'runtime', title: 'Tool Invoked: git-sync', detail: 'Syncing repo state to local buffer.', icon: '🔧', time: 'Just now' }
];

function injectEvents() {
    const container = document.getElementById('liveTimeline');
    container.innerHTML = mockEvents.map(event => `
        <div class="event-card">
            <div class="event-icon">${event.icon}</div>
            <div class="event-content">
                <h4>${event.title}</h4>
                <p>${event.detail}</p>
            </div>
            <div class="event-meta">
                <div>${event.time}</div>
                <div class="runtime-id">${event.component}</div>
            </div>
        </div>
    `).join('');
}

function updateGauge(score) {
    const fill = document.querySelector('.gauge-fill');
    const valueDisp = document.querySelector('.gauge-value');
    
    // Circumference of 40-radius semicircle is approx 125.66 (full circle is 251.32)
    // Here SVG path d="M 20 80 A 40 40 0 1 1 80 80"
    // Approximate length is 188.5 (3/4 of a circle)
    const length = 188.5;
    const offset = length * (1 - score);
    
    fill.style.strokeDashoffset = offset;
    valueDisp.innerText = `${(score * 100).toFixed(1)}%`;
}

// Simple simulation of live updates
function startSimulation() {
    let trust = 0.854;
    
    setInterval(() => {
        // Randomly fluctuate trust score slightly
        trust += (Math.random() - 0.5) * 0.01;
        trust = Math.max(0.7, Math.min(0.99, trust));
        updateGauge(trust);
        
        // Add a random event occasionally
        if (Math.random() > 0.8) {
            const newEvent = {
                title: 'Process Signal',
                detail: `Trust threshold check passed at ${trust.toFixed(2)}`,
                icon: '📡',
                time: 'Just now',
                component: 'runtime'
            };
            
            const container = document.getElementById('liveTimeline');
            const card = document.createElement('div');
            card.className = 'event-card';
            card.style.opacity = '0';
            card.style.transform = 'translateY(-20px)';
            card.innerHTML = `
                <div class="event-icon">${newEvent.icon}</div>
                <div class="event-content">
                    <h4>${newEvent.title}</h4>
                    <p>${newEvent.detail}</p>
                </div>
                <div class="event-meta">
                    <div>${newEvent.time}</div>
                    <div class="runtime-id">${newEvent.component}</div>
                </div>
            `;
            
            container.insertBefore(card, container.firstChild);
            
            // Animation
            setTimeout(() => {
                card.style.transition = '0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275)';
                card.style.opacity = '1';
                card.style.transform = 'translateY(0)';
            }, 10);
            
            // Keep only latest 6 events
            if (container.children.length > 6) {
                container.removeChild(container.lastChild);
            }
        }
    }, 3000);
}

document.addEventListener('DOMContentLoaded', () => {
    injectEvents();
    startSimulation();
    
    // Initialize gauge
    updateGauge(0.854);
    
    // Setup Navigation
    setupNavigation();
});

function setupNavigation() {
    const navItems = document.querySelectorAll('nav li');
    const sections = document.querySelectorAll('.view-section');

    navItems.forEach(item => {
        item.addEventListener('click', () => {
             // Remove active class from all nav items
             navItems.forEach(nav => nav.classList.remove('active'));
             // Add to clicked
             item.classList.add('active');

             // Hide all sections
             sections.forEach(section => section.classList.remove('active-view'));

             // Show target
             const viewId = item.getAttribute('data-view');
             const targetSection = document.getElementById(`view-${viewId}`);
             if (targetSection) {
                 targetSection.classList.add('active-view');
             }
        });
    });
}
