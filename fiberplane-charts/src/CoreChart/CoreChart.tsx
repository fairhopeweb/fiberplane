import { Group } from "@visx/group";
import { Line } from "@visx/shape";
import styled from "styled-components";
import { useContext, useId, useMemo, useState } from "react";

import { ChartContent } from "./ChartContent";
import {
  ChartSizeContext,
  InteractiveControlsState,
  InteractiveControlsStateContext,
  TooltipContext,
} from "../context";
import { Container } from "../BaseComponents";
import { getTimeFormatter } from "../utils";
import { GridWithAxes } from "./GridWithAxes";
import { useMouseControls, useScales, useTooltip } from "../hooks";
import { ZoomBar } from "./ZoomBar";
import type { CoreChartProps } from "./types";

export function CoreChart({
  gridShown = true,
  ...props
}: CoreChartProps &
  Required<Pick<CoreChartProps, "colors">> & {
    gridShown?: boolean;
  }): JSX.Element {
  const { width, height, xMax, yMax, marginTop, marginLeft } =
    useContext(ChartSizeContext);
  const interactiveControlsState = useContext(InteractiveControlsStateContext);

  const { xScaleProps, yScale } = useScales(props);

  const { onMouseDown, onMouseUp, onMouseEnter, onMouseMove, graphContentRef } =
    useMouseControls(props);

  const [shiftKeyPressed, setShiftKeyPressed] = useState(false);

  const onKeyHandler = (event: React.KeyboardEvent) => {
    setShiftKeyPressed(event.shiftKey);
  };

  const onMouseMoveWithShiftDetection = (
    event: React.MouseEvent<HTMLElement>,
  ) => {
    setShiftKeyPressed(event.shiftKey);
    onMouseMove(event);
  };

  const { graphTooltip, showTooltip, hideTooltip } = useTooltip(
    props.showTooltip,
  );
  const tooltipApiValue = useMemo(
    () => ({ showTooltip, hideTooltip }),
    [showTooltip, hideTooltip],
  );

  const clipPathId = useId();

  // Use a custom formatter when `xScale` is a `ScaleBand<number>`. We want to
  // display the time, not the timestamp (number).
  const xScaleFormatter =
    xScaleProps.graphType === "bar" && xScaleProps.stackingType === "none"
      ? getTimeFormatter(xScaleProps.xScale)
      : undefined;

  return (
    <TooltipContext.Provider value={tooltipApiValue}>
      <StyledContainer
        onKeyDown={onKeyHandler}
        onKeyUp={onKeyHandler}
        onMouseDown={onMouseDown}
        onMouseMove={onMouseMoveWithShiftDetection}
        onMouseUp={onMouseUp}
        onMouseEnter={onMouseEnter}
      >
        <svg
          width={width}
          height={height}
          style={{
            cursor: getCursorFromState(
              interactiveControlsState,
              shiftKeyPressed,
            ),
          }}
        >
          <defs>
            <clipPath id={clipPathId}>
              <rect x={0} y={0} width={xMax} height={yMax} />
            </clipPath>
          </defs>
          <Group left={marginLeft} top={marginTop}>
            {gridShown && (
              <GridWithAxes
                xMax={xMax}
                yMax={yMax}
                xScale={xScaleProps.xScale}
                yScale={yScale}
                xScaleFormatter={xScaleFormatter}
                gridColumnsShown={props.gridColumnsShown}
                gridRowsShown={props.gridRowsShown}
                gridBordersShown={props.gridBordersShown}
                gridDashArray={props.gridDashArray}
              />
            )}
            <Group innerRef={graphContentRef} clipPath={`url(#${clipPathId})`}>
              <ChartContent
                timeseriesData={props.timeseriesData}
                xScaleProps={xScaleProps}
                yScale={yScale}
                colors={props.colors}
              />
            </Group>
            <ZoomBar />
          </Group>
          {graphTooltip && (
            <g>
              <Line
                from={{ x: graphTooltip.left, y: 0 }}
                to={{ x: graphTooltip.left, y: yMax }}
                stroke={graphTooltip.color}
                strokeWidth={1}
                pointerEvents="none"
                strokeDasharray="1 1"
              />
              <circle
                cx={graphTooltip.left}
                cy={graphTooltip.top}
                r={4}
                fill={graphTooltip.color}
                pointerEvents="none"
              />
            </g>
          )}
        </svg>
      </StyledContainer>
    </TooltipContext.Provider>
  );
}

const StyledContainer = styled(Container)`
  margin-top: 2px;
`;

function getCursorFromState(
  interactiveControlsState: InteractiveControlsState,
  shiftKey: boolean,
): string {
  switch (interactiveControlsState.type) {
    case "none":
      return shiftKey ? "grab" : "default";
    case "drag":
      return interactiveControlsState.start === null ? "grab" : "grabbing";
    case "zoom":
      return "zoom-in";
  }
}
