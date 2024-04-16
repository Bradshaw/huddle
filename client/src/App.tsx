import React from 'react';
import { useEffect, useState } from 'react';
import './App.css';


function MyButton() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    fetch("/get-count")
      .then((res) => res.json())
      .then((data) => setCount(data.counter));
  }, []);

  function handleClick() {
    fetch("/bump")
      .then((res) => res.json())
      .then((data) => setCount(data.counter));
  }

  return (
    <button onClick={handleClick}>
      Clicked {count} times
    </button>
  );
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
