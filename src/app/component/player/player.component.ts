import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-player',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './player.component.html',
  styleUrls: ['./player.component.css', '../../shared/shared.css']
})
export class PlayerComponent {

}
