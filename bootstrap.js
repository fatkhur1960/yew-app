import './app/main.scss';
import './app/notif.scss';

import("./pkg").then(module => {
  module.run_app();
});