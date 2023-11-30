import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { FormsModule } from "@angular/forms";

import { AppComponent } from "./app.component";
import { NgScrollbarModule } from "ngx-scrollbar";

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, FormsModule, NgScrollbarModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
