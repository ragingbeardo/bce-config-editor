import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-events',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './events.component.html',
  styleUrls: ['./events.component.css', '../../shared/shared.css']
})
export class EventsComponent {

}
