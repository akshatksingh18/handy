import { MicGlyph } from "./MicGlyph";

/** The app icon: a white microphone on the brand colour. */
const AppMark = ({
  width,
  height,
  className,
}: {
  width?: number | string;
  height?: number | string;
  className?: string;
}) => (
  <svg
    width={width ?? 64}
    height={height ?? width ?? 64}
    viewBox="0 0 64 64"
    className={className}
    xmlns="http://www.w3.org/2000/svg"
  >
    <rect
      x={3}
      y={3}
      width={58}
      height={58}
      rx={14}
      fill="var(--color-logo-primary)"
    />
    <MicGlyph k={0.72} color="white" />
  </svg>
);

export default AppMark;
