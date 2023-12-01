import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-raid',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './raid.component.html',
  styleUrls: ['./raid.component.css', '../../shared/shared.css']
})
export class RaidComponent {

}
