import type { HistoryEntry } from "@/bindings";

// Use the exact text delivered to the target app, not the internal raw ASR.
// Old/error entries need an explicit retry or raw-recovery opt-in in Hinglish mode.
export function historyDeliveryText(
  entry: Pick<HistoryEntry, "transcription_text" | "post_processed_text">,
  romanHinglish: boolean,
): string | undefined {
  const text = entry.post_processed_text ?? entry.transcription_text;
  return romanHinglish && /[\u0900-\u097f]/u.test(text) ? undefined : text;
}
