import styled from "@emotion/styled";
import React from "react";
import { tauri, dialog } from "../tauri";

let Btn = styled.div`
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 43px;
  line-height: 43px;
  text-align: center;
  font-size: 14px;
  color: rgb(72, 72, 93);
  font-weight: bolder;
  background: linear-gradient(to right, rgb(180, 161, 243), rgb(184, 228, 251));
  cursor: pointer;
  user-select: none;
`;

function AddBtn() {
  async function add() {
    // const confirmed2 = await dialog.confirm(
    //   "This action cannot be reverted. Are you sure?",
    //   { title: "Tauri", type: "warning" }
    // );
    // console.log(confirmed2);
    tauri.invoke("alert");
  }
  return <Btn onClick={add}>添加</Btn>;
}

export { AddBtn };
