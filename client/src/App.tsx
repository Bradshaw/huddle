import React from 'react';
import logo from './logo.png';
import { useEffect, useState } from 'react';
import './App.css';

enum StateType {
  Unset,
  Error,
  State,
}

type Unset = {
  type: StateType.Unset,
}

type State = {
  type: StateType.State,
  count: number;
}

type StateError = {
  type: StateType.Error,
  error: Error
}

type NetworkState = 
    | State
    | StateError
    | Unset;


function MyButton() {
  const [state, setState] = useState({type: StateType.Unset} as NetworkState);

  useEffect(() => {
    fetch("/api/counter")
      .then((res) => res.json())
      .then((data) => setState({
          type: StateType.State,
          count: data.count
        }))
      .catch(error => 
        setState({
          type: StateType.Error,
          error: error
        }));
  }, []);

  function handleClick() {
    fetch("/api/counter", {method: "POST"})
      .then((res) => res.json())
      .then((data) => setState({
          type: StateType.State,
          count: data.count
        }))
      .catch(error => 
        setState({
          type: StateType.Error,
          error: error
        }));
  }

  switch (state.type) {
    case StateType.Unset:
      return <div><p>Loading...</p></div>
    case StateType.Error:
      return (
      <div>
        <p>Could not load current count...</p>
        <button onClick={handleClick}>
          Try again?
        </button>
      </div>
      )
    default:
      return (
        <div>
          <p>Button was clicked {state.count} times</p>
          <button onClick={handleClick}>
            bump
          </button>
        </div>
      )
  }
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <MyButton />
      </header>
    </div>
  );
}

export default App;
