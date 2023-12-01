import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

enum Menu {
  SETTINGS = 1,
  STORAGE = 2,
  HIDEOUT = 3,
  TRADERS = 4,
  ITEMS = 5,
  PLAYER = 6,
  SCAV = 7,
  RAID = 8,
  EVENTS = 9,
}

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent {
  jsonConfig: any;
  menu = Menu;
  activeMenu: Menu = this.menu.SETTINGS;

  setActiveMenu(menu: Menu): void {
    this.activeMenu = menu;
  }

  readCurrentConfig(): void {
    invoke<string>("read_current_config", {}).then((text) => {
      this.jsonConfig = JSON.parse(text);
    });
  }

  async writeNewConfig(): Promise<void> {
    const jsonData = JSON.stringify(this.jsonConfig);

    try {
      await invoke("write_new_config", { jsonData });
      console.log("Config successfully written");
    } catch (error) {
      console.error("Error writing config:", error);
    }
  }
}
