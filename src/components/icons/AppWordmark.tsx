import { useTranslation } from "react-i18next";
import AppMark from "./AppMark";

/** The app mark beside its name, for the sidebar and the welcome screen. */
const AppWordmark = ({ className }: { className?: string }) => {
  const { t } = useTranslation();
  return (
    <div className={`flex items-center gap-2 ${className ?? ""}`}>
      <AppMark width={28} className="shrink-0" />
      <span className="text-sm font-semibold leading-tight">
        {t("appName")}
      </span>
    </div>
  );
};

export default AppWordmark;
