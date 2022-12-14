/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: App.tsx                              */
/*       ## ##                                                */
/*                    C: 2022/09/20 11:16:30 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/21 15:25:40 by:dnettoRaw     */
/*    ###########                                             */ 

import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import logo from '../assets/logo.svg'
import '../css/App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React! lops</p>
        <p>
          <button type="button" onClick={() => invoke('test')}> let's print in terminal</button>
          <button type="button" onClick={() => invoke('create_window')}> let's print in terminal</button>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
        </p>
        <p>
          Edit <code>App.tsx</code> and save to test HMR updates.
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
