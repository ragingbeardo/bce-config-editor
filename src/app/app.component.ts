import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent {
  
  jsonConfig: any;

  readCurrentConfig(): void {
    invoke<string>("read_current_config", {}).then((text) => {
      this.jsonConfig = JSON.parse(text);
    });
  }

  async writeNewConfig(): Promise<void> {
    const jsonString = JSON.stringify(this.jsonConfig);

    try {
      await invoke('write_new_config', { json_data: jsonString });
      console.log('Config successfully written');
    } catch (error) {
      console.error('Error writing config:', error);
    }
  }
}
