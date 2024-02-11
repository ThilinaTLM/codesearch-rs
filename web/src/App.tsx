import {SearchPage} from "@/pages";
import {useEffect} from "react";
import api from "@/api";

function App() {

  useEffect(() => {
    api.health().then((res) => {
      console.log(`Server health: ${res.data.status}`);
    })
  })

  return (
    <>
      <SearchPage />
    </>
  )
}

export default App
