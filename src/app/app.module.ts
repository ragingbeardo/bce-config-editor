import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { SettingsComponent } from "./component/settings/settings.component";
import { StorageComponent } from "./component/storage/storage.component";
import { HideoutComponent } from "./component/hideout/hideout.component";
import { TradersComponent } from "./component/traders/traders.component";
import { ItemsComponent } from "./component/items/items.component";
import { PlayerComponent } from "./component/player/player.component";
import { ScavComponent } from "./component/scav/scav.component";
import { RaidComponent } from "./component/raid/raid.component";
import { EventsComponent } from "./component/events/events.component";

@NgModule({
  declarations: [AppComponent],
  imports: [
    BrowserModule,
    SettingsComponent,
    StorageComponent,
    HideoutComponent,
    TradersComponent,
    ItemsComponent,
    PlayerComponent,
    ScavComponent,
    RaidComponent,
    EventsComponent,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
