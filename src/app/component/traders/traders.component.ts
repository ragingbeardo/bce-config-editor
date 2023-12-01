import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-traders',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './traders.component.html',
  styleUrls: ['./traders.component.css', '../../shared/shared.css']
})
export class TradersComponent {

}
