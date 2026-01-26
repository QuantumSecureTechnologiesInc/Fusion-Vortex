package main

import (
	"github.com/charmbracelet/lipgloss"
)

var HelpText = `
VORTEX v2.0 - OFFICIAL DOCUMENTATION
====================================

1. OVERVIEW
-----------
HyperCycle Vortex is a post-quantum cryptographic suite secured by 
Chaos Dynamics. Unlike traditional RNGs, Vortex uses the chaotic 
divergence of nonlinear systems to generate entropy that is 
mathematically provable to be unpredictable.

2. CLI USAGE
------------
This terminal client serves as the primary control center for your 
Vortex node. 

[TAB]   - Switch Views (Dashboard / 2FA / Settings / Help)
[Q]     - Quit Application

3. 2FA AUTHENTICATION
---------------------
The '2FA' tab generates Time-based One-Time Passwords (TOTP) 
synced with your Web Dashboard. In 'Secure Mode', the dashboard 
requires this code for all login attempts.

4. CHAOS METRICS
----------------
Lyapunov Exponent (LLE): Measures the rate of separation of 
infinitesimally close trajectories. Positive LLE indicates chaos.
Target: > 0.05

Entropy Rate: The speed at which random bits are generated and 
pushed to the pool.

5. SUPPORT
----------
For enterprise support, contact: security@hypercycle.ai
`

func RenderHelp() string {
	return ContainerStyle.
		Width(70).
		Render(
			TitleStyle.Render("HELP & DOCUMENTATION") + "\n\n" +
				lipgloss.NewStyle().Foreground(lipgloss.Color("#DDD")).Render(HelpText),
		)
}
