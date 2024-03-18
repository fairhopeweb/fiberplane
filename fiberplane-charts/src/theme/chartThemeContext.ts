import { createContext } from "react";
import type { Theme } from "../chartThemeTypes";

export const defaultChartTheme: Theme = {
  axisColor: "#a4a4a4",
  axisFontFamily: "inherit",
  axisFontSize: "12px",
  axisFontStyle: "normal",
  axisFontWeight: "400",
  axisLetterSpacing: "inherit",
  buttonActiveBackgroundColor: "#606266",
  buttonActiveColor: "white",
  buttonBackgroundColor: "transparent",
  buttonBorderRadius: "6px",
  buttonColor: "#1f2023",
  buttonDisabledBackgroundColor: "transparent",
  buttonDisabledColor: "#a4a4a4",
  buttonFocusBackgroundColor: "white",
  buttonFocusBorderColor: "#285fff",
  buttonFocusColor: "#606266",
  buttonFocusOutline: "rgb(183, 201, 255) solid 2px",
  buttonFont: "normal 600 14px / 16px sans-serif",
  buttonGroupBackgroundColor: "#f3f3f3",
  buttonGroupBorderRadius: "6px",
  buttonHoverBackgroundColor: "#e7e7e7",
  buttonHoverColor: "#1f2023",
  eventColor: "#285fff",
  expandableGradientColor: "rgb(255 255 255 / 75%)",
  gridStrokeColor: "#e7e7e7",
  legendItemBorderRadius: "6px",
  legendItemCheckboxBorderRadius: "4px",
  legendItemCheckboxColor: "#285fff",
  legendItemColor: "black",
  legendItemEmphasisBackgroundColor: "#f3f3f3",
  legendItemEmphasisBorderRadius: "4px",
  legendItemEmphasisColor: "black",
  legendItemEmphasisFont: "inherit",
  legendItemFont: "inherit",
  legendItemOnHoverBackgroundColor: "#a4a4a4",
  legendItemOnHoverColor: "inherit",
  legendResultsColor: "inherit",
  legendResultsFont: "inherit",
  legendResultsLetterSpacing: "inherit",
  shapeListColors: ["#c00eae", "#23304a", "#cf3411"],
  targetLatencyColor: "inherit",
};

export const ChartThemeContext = createContext<Theme>(defaultChartTheme);
