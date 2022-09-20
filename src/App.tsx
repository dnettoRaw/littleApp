/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: App.tsx                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:15:07 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/18 13:18:42 by:dnettoRaw     */
/*    ###########                                             */

import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import logo from './logo.svg'
import './App.css'

// function runCommand(commandName: any, args: any, optional: any) {
//   const id = optional ? '#response-optional' : '#response'
//   const result = document.querySelector(id)
//   window.__TAURI__
//     .invoke(commandName, args)
//     .then((response) => {
//       result.innerText = `Ok(${response})`
//     })
//     .catch((error) => {
//       result.innerText = `Err(${error})`
//     })
// }


function App() {
  const [count, setCount] = useState(0);
  const invoked = window.__TAURI__.invoke;

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
          <button type="button" onClick={() => invoked('my_button')}> let's print in terminal</button>
          <br/>
          <input name="mytext" type="text"></input>
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
