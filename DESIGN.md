---
version: alpha
name: SIMDays Glass
description: A vivid, readable glassmorphism design system for the compact SIMDays desktop app. Visual redesign only; all existing behavior and content remain intact.
colors:
  primary: "#2563EB"
  primary-pressed: "#1D4ED8"
  on-primary: "#FFFFFF"
  primary-text: "#315FAE"
  action-glass: "rgba(239, 246, 255, 0.76)"
  action-glass-pressed: "rgba(222, 235, 255, 0.86)"
  ink: "#172033"
  ink-secondary: "#46536A"
  ink-muted: "#66758C"
  canvas: "#E8F1FF"
  canvas-deep: "#DCE9FF"
  titlebar-pearl: "#F4F7FC"
  titlebar-text: "#29364B"
  titlebar-border: "#E0E9F4"
  glass-main: "rgba(255, 255, 255, 0.58)"
  glass-soft: "rgba(255, 255, 255, 0.38)"
  glass-strong: "rgba(255, 255, 255, 0.76)"
  glass-border: "rgba(255, 255, 255, 0.78)"
  glass-highlight: "rgba(255, 255, 255, 0.92)"
  glass-shadow: "rgba(41, 72, 130, 0.16)"
  blue-wash: "#B9D4FF"
  lilac-wash: "#D9C9FF"
  aqua-wash: "#B8F0EA"
  normal: "#16835D"
  normal-tint: "rgba(22, 131, 93, 0.12)"
  soon: "#A86408"
  soon-tint: "rgba(225, 154, 38, 0.16)"
  danger: "#C43B4D"
  danger-tint: "rgba(196, 59, 77, 0.14)"
  divider: "rgba(40, 65, 110, 0.10)"
  white: "#FFFFFF"
  transparent: "transparent"
typography:
  display:
    fontFamily: "Segoe UI Variable Display, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 68px
    fontWeight: 700
    lineHeight: 1.0
    letterSpacing: -2px
    fontFeature: "tnum"
  display-compact:
    fontFamily: "Segoe UI Variable Display, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 48px
    fontWeight: 700
    lineHeight: 1.05
    letterSpacing: -1.4px
    fontFeature: "tnum"
  title:
    fontFamily: "Segoe UI Variable Display, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 22px
    fontWeight: 650
    lineHeight: 1.25
    letterSpacing: -0.35px
  section-title:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 16px
    fontWeight: 650
    lineHeight: 1.35
  body:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 400
    lineHeight: 1.5
  body-strong:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 600
    lineHeight: 1.4
  label:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 12px
    fontWeight: 550
    lineHeight: 1.4
  caption:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 11px
    fontWeight: 400
    lineHeight: 1.45
  button:
    fontFamily: "Segoe UI Variable Text, Segoe UI, Microsoft YaHei UI, system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 650
    lineHeight: 1.2
rounded:
  xs: 8px
  sm: 12px
  md: 16px
  lg: 22px
  xl: 28px
  pill: 9999px
spacing:
  xxs: 4px
  xs: 8px
  sm: 12px
  md: 16px
  lg: 20px
  xl: 24px
  xxl: 32px
  section: 40px
components:
  status-card-normal:
    backgroundColor: "{colors.glass-main}"
    textColor: "{colors.ink}"
    typography: "{typography.display}"
    rounded: "{rounded.xl}"
    padding: 24px
  status-card-soon:
    backgroundColor: "{colors.glass-main}"
    textColor: "{colors.ink}"
    typography: "{typography.display}"
    rounded: "{rounded.xl}"
    padding: 24px
  status-card-danger:
    backgroundColor: "{colors.glass-main}"
    textColor: "{colors.ink}"
    typography: "{typography.display}"
    rounded: "{rounded.xl}"
    padding: 24px
  status-chip-normal:
    backgroundColor: "{colors.normal-tint}"
    textColor: "{colors.normal}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: 6px 11px
  status-chip-soon:
    backgroundColor: "{colors.soon-tint}"
    textColor: "{colors.soon}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: 6px 11px
  status-chip-danger:
    backgroundColor: "{colors.danger-tint}"
    textColor: "{colors.danger}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: 6px 11px
  info-glass-card:
    backgroundColor: "{colors.glass-soft}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.lg}"
    padding: 8px 18px
  button-primary:
    backgroundColor: "{colors.action-glass}"
    textColor: "{colors.primary-text}"
    typography: "{typography.button}"
    rounded: "{rounded.md}"
    padding: 14px 18px
  button-primary-pressed:
    backgroundColor: "{colors.action-glass-pressed}"
    textColor: "{colors.primary-text}"
    typography: "{typography.button}"
    rounded: "{rounded.md}"
    padding: 14px 18px
  button-secondary:
    backgroundColor: "{colors.glass-soft}"
    textColor: "{colors.ink-secondary}"
    typography: "{typography.button}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  setup-glass-card:
    backgroundColor: "{colors.glass-main}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.xl}"
    padding: 24px
  settings-glass-panel:
    backgroundColor: "{colors.glass-strong}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.xl}"
    padding: 22px
  input-glass:
    backgroundColor: "{colors.glass-strong}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.sm}"
    padding: 11px 12px
  reminder-glass-card:
    backgroundColor: "{colors.glass-main}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.xl}"
    padding: 20px
  toast-glass:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.white}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: 10px 16px
---

## Overview

SIMDays is a small Windows desktop utility for tracking a recurring SIM reset date. Its visual identity is **bright, tactile glass over a softly colored atmosphere**: a compact utility that feels polished and buoyant, while the countdown stays immediately readable. The main countdown card is a prominent translucent glass object, not a flat white card. The surrounding canvas supplies visible blue, lilac, and aqua color fields so transparency and blur are perceptible even in a 380px-wide window.

This design system applies to the existing setup view, countdown view, settings sheet, toast, and reminder window. It changes visual presentation only. Preserve all current text, controls, status meanings, validation, keyboard and pointer behavior, Tauri commands, reminder timing, tray behavior, and stored data. Do not add screens, fields, actions, or new product behavior.

## Colors

The canvas is a cool, luminous blue-white with large soft radial color washes in blue, lilac, and aqua. These washes are part of the background behind glass and must remain visible around and through panels. Use gradients only for these ambient background washes and subtle state glows; do not use gradients on text or primary buttons.

The native Windows title bar uses `{colors.titlebar-pearl}` with `{colors.titlebar-text}` for its caption and controls, plus a fine `{colors.titlebar-border}` frame. This pearl surface bridges the Windows chrome to the pale glass canvas without creating another saturated band.

Glass surfaces use white with visible transparency. The main countdown card uses `{colors.glass-main}`; secondary information uses `{colors.glass-soft}`; the settings panel and inputs use `{colors.glass-strong}` for dependable contrast. Pair each surface with a fine white edge, a restrained inner highlight along its top edge, and dark blue-gray text. Keep underlying background detail soft and low contrast so glass never reduces legibility.

Blue is the primary action accent, used as clear text and focus color on a pale frosted action surface. Keep the main action distinct through its stronger edge and shadow while allowing it to sit naturally inside the glass palette. Preserve the existing state semantics: green means normal, amber means approaching the reminder threshold, and red means due or overdue. State must remain identifiable through the existing label and text as well as color. Use each state hue as a restrained glow or chip tint, never as the full card fill.

## Typography

Use the system UI stack, preferring Segoe UI Variable on Windows and falling back to Segoe UI, Microsoft YaHei UI, Microsoft YaHei, and system-ui. Use tabular numerals for dates and the day count so values do not visually jump. The countdown is the strongest typographic element; keep its numeral large and heavy, with the unit smaller and baseline-aligned. For the special `今天` state, use `{typography.display-compact}` so it fits without changing its meaning.

Use dark ink for primary content and muted slate for supporting labels. Keep the app title, status label, date rows, helper copy, and button labels at the compact sizes defined in tokens. Do not reduce functional text below 11px or use thin weights for small labels.

## Layout

The main window is a compact, vertical desktop utility with a 380 × 660px starting size and a minimum of 360 × 560px. Design for this narrow width first. Keep the existing content order: status/countdown, date details, reset action, settings action. Maintain a consistent 16–24px outer inset and 12–16px gaps. Avoid web landing-page layouts, wide dashboards, navigation bars, and oversized hero spacing.

The countdown card occupies the visual center of the main view and should feel layered above the ambient canvas. Its status chip sits above the number; the countdown label sits below. The information card remains a compact three-row list with subtle dividers. The reset button is the clearest action and spans the available content width. Settings remains visually secondary.

The setup view uses the same glass language in a single centered vertical flow. Keep the app identity and explanatory text above the existing date and interval inputs, then the existing primary action. The settings sheet covers the view with a stronger, more opaque glass surface; preserve its current field order and allow vertical scrolling when needed. The reminder window stays a compact centered alert with existing icon, title, supporting text, and two actions.

Use the available width rather than introducing horizontal scrolling. At the minimum window size, reduce outer padding and vertical gaps before shrinking controls or text. Inputs and buttons should have at least 42px height. Keep existing date/number input semantics and native picker affordances.

## Elevation & Depth

Create obvious glass depth with three layers: a softly colored ambient canvas, a translucent surface with `backdrop-filter: blur(22px) saturate(155%)`, and a broad cool-tinted shadow below the main card. Use a 1px translucent white edge and a subtle inset highlight to catch light. Secondary panels use a lighter shadow and a lower blur strength around 14–18px. The main countdown card can use the strongest blur and depth in the interface.

The surface behind the main window is app-owned artwork: use CSS radial gradients rather than relying on the desktop wallpaper. This keeps the material visible and consistent in the ordinary Tauri window. The undecorated reminder window must render a self-contained gradient canvas and glass panel; do not assume CSS `backdrop-filter` can blur content outside its native webview window. When backdrop filtering is unavailable, use a more opaque surface token and retain the edge, highlight, and shadow so the interface remains legible.

Do not apply large shadows to every control. The status card and settings panel carry the strongest elevation; inputs, chips, and buttons rely mainly on surface contrast and edge highlights.

## Shapes

Use rounded, soft geometry: 28px for the main glass card and settings panel, 22px for secondary glass cards, 12–16px for controls, and pill shapes for state chips. Maintain a consistent radius within each component. Avoid sharp rectangular panels and avoid making every element a pill.

## Components

**Status card** — A large translucent panel with generous internal padding, soft white rim, top-edge highlight, and cool shadow. The green, amber, or red state changes the chip and a narrow edge/accent glow. Keep the card body glass-white in all states. Preserve the current status text, countdown value, unit, and explanatory label. Do not use color alone to communicate status.

**Information card** — A quiet frosted panel containing the existing next reset, last reset, and interval rows. Align labels left and values right. Use tabular numerals for values and thin dividers with `{colors.divider}`. Keep this readable and subordinate to the countdown card.

**Primary button** — Pale icy-blue translucent glass with semibold blue text, a fine white rim, rounded 16px corners, and a restrained cool shadow. It is the visually clearest action through its edge and depth, without a heavy saturated fill. It is used for the existing main action on setup, reset, and save. Pressed state uses `{colors.action-glass-pressed}` and a slight scale to 0.98. Preserve exact button labels and click behavior.

**Secondary button** — A translucent glass control with a subtle white edge and dark slate text. Use for settings, dismiss, and close actions where appropriate. Do not give it the same visual weight as the primary action.

**Inputs** — Use a bright, mostly opaque glass field with dark text, a fine translucent border, and a clear blue focus ring. Keep labels and helper text outside the field. Preserve native input types, validation feedback, and all existing bounds.

**Settings panel** — A full-view overlay with a stronger blur and higher surface opacity than the main card. Keep its title and close control at the top, the current fields in their existing order, and the save button after the fields. Focus and error states must remain easy to identify against the light surface.

**Reminder alert** — A compact glass alert over its own cool gradient canvas. Preserve the existing level-dependent icon, title and detail text and both actions. Use the current status tint sparingly around the icon or as a small badge; keep the reset action visually primary.

**Toast** — A dark translucent pill with white text, soft blur, and a subtle shadow. Keep it legible over the gradient and glass layers.

## Do's and Don'ts

### Do

- Make the countdown card visibly translucent; do not render it as a plain opaque white card.
- Put blue, lilac, and aqua ambient color behind glass so blur and transparency are visibly present.
- Keep text dark and high contrast on light glass.
- Preserve green, amber, and red as the existing status meanings, including both color and text labels.
- Use stronger glass opacity for settings and input fields where readability matters most.
- Keep all current controls, labels, states, and actions in their current functional roles.
- Use CSS-only decoration and existing web technologies; the app has no frontend package dependencies.

### Don't

- Don't alter Rust commands, JavaScript behavior, validation, state transitions, data storage, reminder cadence, tray behavior, or window actions.
- Don't add a dark mode, new navigation, charts, onboarding steps, fields, or new interactions.
- Don't put translucent text directly over busy or high-contrast imagery.
- Don't use bright rainbow gradients, neon glows, heavy blur that smears the interface, or a separate drop shadow on every element.
- Don't remove or recolor the status labels in a way that changes their meanings.
- Don't depend on transparency to the Windows desktop; the main glass effect is layered over an app-owned CSS background.
- Don't introduce horizontal scrolling or change the compact utility layout into a marketing page.
