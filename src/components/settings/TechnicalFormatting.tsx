import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface TechnicalFormattingProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const TechnicalFormatting: React.FC<TechnicalFormattingProps> =
  React.memo(({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();
    const enabled = getSetting("technical_formatting_enabled") ?? true;

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={(nextEnabled) =>
          updateSetting("technical_formatting_enabled", nextEnabled)
        }
        isUpdating={isUpdating("technical_formatting_enabled")}
        label={t("settings.advanced.technicalFormatting.title")}
        description={t("settings.advanced.technicalFormatting.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  });
