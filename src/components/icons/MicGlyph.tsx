/** The app's microphone mark in a 64-unit box, shared by the tray icon and the app mark. */
const STROKE = {
  fill: "none",
  strokeWidth: 4.5,
  strokeLinecap: "round" as const,
};

/** Microphone, scaled by `k` about the box centre. */
export const MicGlyph = ({
  k = 1,
  color = "currentColor",
}: {
  k?: number;
  color?: string;
}) => (
  <g transform={`translate(32 32) scale(${k}) translate(-32 -32)`}>
    <rect x={24} y={6} width={16} height={30} rx={8} fill={color} />
    <path d="M46 30 A14 14 0 0 1 18 30" {...STROKE} stroke={color} />
    <path d="M32 44 V54 M23 55 H41" {...STROKE} stroke={color} />
  </g>
);
