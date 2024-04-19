import React from 'react';
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
    fetch("/api/get-count")
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
    fetch("/api/bump")
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
        <MyButton />
      </header>
    </div>
  );
}

export default App;
