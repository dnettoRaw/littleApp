/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: main.tsx                             */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:14:52 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/18 13:18:19 by:dnettoRaw     */
/*    ###########                                             */

import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

declare global {
  interface Window {
      __TAURI__: Record<string, any>;
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
