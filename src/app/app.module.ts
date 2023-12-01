import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { SettingsComponent } from "./component/settings/settings.component";
import { StorageComponent } from "./component/storage/storage.component";

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, SettingsComponent, StorageComponent],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
