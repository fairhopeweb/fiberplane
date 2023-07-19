import { AreaShape } from "./AreaShape";
import type { CommonShapeProps } from "./types";
import { LineShape } from "./LineShape";
import { PointShape } from "./PointShape";
import { RectangleShape } from "./RectangleShape";
import type { Shape } from "../../Mondrian";

type Props<P> = CommonShapeProps & { shape: Shape<P> };

export function ChartShape<P>({ shape, ...props }: Props<P>): JSX.Element {
  switch (shape.type) {
    case "area":
      return <AreaShape area={shape} {...props} />;

    case "line":
      return <LineShape line={shape} {...props} />;

    case "point":
      return <PointShape point={shape} {...props} />;

    case "rectangle":
      return <RectangleShape rectangle={shape} {...props} />;
  }
}
