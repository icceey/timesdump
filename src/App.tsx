import { useEffect, useState } from "react";
import HudView from "./components/HudView";
import SettingsView from "./components/SettingsView";

function App() {
  const [view, setView] = useState<"hud" | "settings">("hud");

  useEffect(() => {
    // Determine which view to show based on URL hash
    const hash = window.location.hash;
    if (hash.includes("settings")) {
      setView("settings");
    } else {
      setView("hud");
    }
  }, []);

  return (
    <div className="h-full">
      {view === "hud" ? <HudView /> : <SettingsView />}
    </div>
  );
}

export default App;
