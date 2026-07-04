## 2025-07-04 - Array Recreation on Keystroke
**Learning:** In React components with form inputs (`intent` in `fusion-visual-ui/app/page.tsx`), keeping static arrays containing objects and JSX inside the component body causes them to be recreated on every single keystroke. This wastes memory and triggers deeper equality checks during reconciliation.
**Action:** Always extract static configurations, lists, and arrays outside of the component body to preserve their references across renders, especially in components with high-frequency state updates like typing.
