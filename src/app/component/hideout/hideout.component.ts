import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-hideout',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './hideout.component.html',
  styleUrls: ['./hideout.component.css', '../../shared/shared.css']
})
export class HideoutComponent {

}
