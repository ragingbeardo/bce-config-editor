import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent {
  data = "";

  readCurrentConfig(): void {
    invoke<string>("read_current_config", {}).then((text) => {
      this.data = text;
      const jsonObject = JSON.parse(text);
      console.log(jsonObject);
    });
  }

  async writeNewConfig(jsonData: string): Promise<void> {
    try {
      await invoke('write_new_config', { json_data: jsonData });
      console.log('Config successfully written');
    } catch (error) {
      console.error('Error writing config:', error);
    }
  }
}
