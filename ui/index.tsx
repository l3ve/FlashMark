// import styled from "@emotion/styled";
import React, { useEffect } from "react";
import { createRoot } from "react-dom/client";
// import { AddBtn } from "./add";

function Box() {
  useEffect(() => {
    console.log("333");
  }, []);
  return <>Flash Mark!</>;
  // return <AddBtn />;
}

let body = document.getElementById("flash-mark");
if (!body) {
  body = document.createElement("div");
  body.id = "flash-mark";
  document.body.appendChild(body);
}

const root = createRoot(body);
root.render(<Box />);
