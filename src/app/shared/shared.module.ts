import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NgScrollbarModule } from 'ngx-scrollbar';
import { FormsModule } from '@angular/forms';

@NgModule({
  declarations: [],
  imports: [
    CommonModule,
    FormsModule,
    NgScrollbarModule
  ],
  exports: [
    CommonModule,
    FormsModule,
    NgScrollbarModule
  ]
})
export class SharedModule { }
