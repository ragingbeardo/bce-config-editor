import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-items',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './items.component.html',
  styleUrls: ['./items.component.css', '../../shared/shared.css']
})
export class ItemsComponent {

}
