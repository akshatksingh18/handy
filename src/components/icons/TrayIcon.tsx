/** The tray icon, drawn from the same 64-unit geometry as src-tauri/resources/tray_*.png; uses currentColor like a macOS template image. */
import { useId } from "react";
import { MicGlyph } from "./MicGlyph";

type TrayIconState = "idle" | "recording" | "transcribing";

const BARS: [number, number][] = [
  [19, 10],
  [25.5, 20],
  [32, 30],
  [38.5, 20],
  [45, 10],
];

const TrayIcon = ({
  state,
  size,
}: {
  state: TrayIconState;
  size: number | string;
}) => {
  const maskId = useId();
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      className="fill-current"
      xmlns="http://www.w3.org/2000/svg"
    >
      {state === "idle" ? (
        <MicGlyph />
      ) : (
        <>
          <mask id={maskId}>
            <circle cx={32} cy={32} r={30} fill="white" />
            {state === "recording" ? (
              <MicGlyph k={0.62} color="black" />
            ) : (
              BARS.map(([x, h]) => (
                <rect
                  key={x}
                  x={x - 2.4}
                  y={32 - h / 2}
                  width={4.8}
                  height={h}
                  rx={2.4}
                  fill="black"
                />
              ))
            )}
          </mask>
          <circle cx={32} cy={32} r={30} mask={`url(#${maskId})`} />
        </>
      )}
    </svg>
  );
};

export default TrayIcon;
