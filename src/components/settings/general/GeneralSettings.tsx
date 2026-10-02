import React from "react";
import { useTranslation } from "react-i18next";
import { type } from "@tauri-apps/plugin-os";
import { MicrophoneSelector } from "../MicrophoneSelector";
import { ChannelSelector } from "../ChannelSelector";
import { ShortcutInput } from "../ShortcutInput";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { OutputDeviceSelector } from "../OutputDeviceSelector";
import { ShortcutActivationSetting } from "../ShortcutActivation";
import { AudioFeedback } from "../AudioFeedback";
import { useSettings } from "../../../hooks/useSettings";
import { VolumeSlider } from "../VolumeSlider";
import { MuteWhileRecording } from "../MuteWhileRecording";
import { ModelSettingsCard } from "./ModelSettingsCard";
import { ToggleSwitch } from "../../ui/ToggleSwitch";

export const GeneralSettings: React.FC = () => {
  const { t } = useTranslation();
  const { audioFeedbackEnabled, getSetting, updateSetting, isUpdating } =
    useSettings();
  const isLinux = type() === "linux";
  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("settings.general.title")}>
        <ShortcutInput shortcutId="transcribe" grouped={true} />
        <ShortcutActivationSetting descriptionMode="tooltip" grouped={true} />
        {/* Cancel shortcut remains hidden on Linux because of dynamic shortcut instability. */}
        {!isLinux && <ShortcutInput shortcutId="cancel" grouped={true} />}
      </SettingsGroup>
      <ModelSettingsCard />
      <SettingsGroup title={t("settings.voiceWorkspace.title")}>
        <ToggleSwitch
          checked={getSetting("roman_hinglish_enabled") ?? false}
          onChange={(enabled) =>
            updateSetting("roman_hinglish_enabled", enabled)
          }
          isUpdating={isUpdating("roman_hinglish_enabled")}
          label={t("settings.voiceWorkspace.romanHinglish.label")}
          description={t("settings.voiceWorkspace.romanHinglish.description")}
          descriptionMode="inline"
          grouped={true}
        />
        <ToggleSwitch
          checked={getSetting("preload_model") ?? false}
          onChange={(enabled) => updateSetting("preload_model", enabled)}
          isUpdating={isUpdating("preload_model")}
          label={t("settings.voiceWorkspace.preloadModel.label")}
          description={t("settings.voiceWorkspace.preloadModel.description")}
          descriptionMode="inline"
          grouped={true}
        />
      </SettingsGroup>
      <SettingsGroup title={t("settings.sound.title")}>
        <MicrophoneSelector descriptionMode="tooltip" grouped={true} />
        <ChannelSelector descriptionMode="tooltip" grouped={true} />
        <MuteWhileRecording descriptionMode="tooltip" grouped={true} />
        <AudioFeedback descriptionMode="tooltip" grouped={true} />
        <OutputDeviceSelector
          descriptionMode="tooltip"
          grouped={true}
          disabled={!audioFeedbackEnabled}
        />
        <VolumeSlider disabled={!audioFeedbackEnabled} />
      </SettingsGroup>
    </div>
  );
};
