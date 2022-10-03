import {useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog'
import "./App.css";

function App() {
  useEffect(() => {
    invoke('get_video_files').then((res) => {
      console.log(res)
    })
  }, [])
  
  const selectDirectory = async () => {
    try {
      const res = await open({
        multiple: false,
        directory: true,
      }).then((res) => {
        console.log(res)
      })
    } catch (err) {
      console.log(err)
    }
  }

  return (
    <div className="container">
      ANIME
      <button onClick={selectDirectory}>select dir</button>
    </div>
  );
}

export default App;
