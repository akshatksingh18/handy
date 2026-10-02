import { strict as assert } from "node:assert";
import { historyDeliveryText } from "./deliveryText";

assert.equal(
  historyDeliveryText(
    { transcription_text: "आज API टेस्ट", post_processed_text: "Aaj API test" },
    true,
  ),
  "Aaj API test",
);
assert.equal(
  historyDeliveryText(
    { transcription_text: "आज API टेस्ट", post_processed_text: null },
    true,
  ),
  undefined,
);
assert.equal(
  historyDeliveryText(
    { transcription_text: "आज API टेस्ट", post_processed_text: null },
    false,
  ),
  "आज API टेस्ट",
);
assert.equal(
  historyDeliveryText(
    { transcription_text: "API test", post_processed_text: null },
    true,
  ),
  "API test",
);
assert.equal(
  historyDeliveryText(
    { transcription_text: "raw", post_processed_text: "आज" },
    true,
  ),
  undefined,
);
assert.equal(
  historyDeliveryText(
    { transcription_text: "raw", post_processed_text: "Cleaned." },
    false,
  ),
  "Cleaned.",
);
console.log("History delivery regression checks passed (6).");
