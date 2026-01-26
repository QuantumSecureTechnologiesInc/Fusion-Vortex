package main

import (
	"github.com/charmbracelet/lipgloss"
)

// --- Color Palette (User Defined) ---
var (
	GradientColors = []string{
		"#F54927", "#EB5331", "#E15B3A", "#D76243", "#CC684C",
		"#C16D55", "#B6725D", "#AA7666", "#9D7A6F", "#8F7D77", "#808080",
	}

	PrimaryColor   = lipgloss.Color("#F54927") // Hero Orange
	SecondaryColor = lipgloss.Color("#808080") // Grey
	DarkBg         = lipgloss.Color("#1a1f3a") // Deep Navy/Black

	// Text Styles
	TitleStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(PrimaryColor).
			MarginBottom(1)

	SubTitleStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#AA7666")).
			Italic(true)

	// Box Styles
	ContainerStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(PrimaryColor).
			Padding(1, 2).
			Align(lipgloss.Left)

	// Tab Styles
	ActiveTabStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(PrimaryColor).
			Padding(0, 1).
			Foreground(PrimaryColor).
			Bold(true)

	InactiveTabStyle = lipgloss.NewStyle().
				Border(lipgloss.RoundedBorder()).
				BorderForeground(SecondaryColor).
				Padding(0, 1).
				Foreground(SecondaryColor)
)

// --- Helper Functions ---

func RenderGradientText(text string) string {
	var result string
	for i, char := range text {
		idx := i
		if idx >= len(GradientColors) {
			idx = len(GradientColors) - 1
		}
		style := lipgloss.NewStyle().Foreground(lipgloss.Color(GradientColors[idx])).Bold(true)
		result += style.Render(string(char))
	}
	return result
}
