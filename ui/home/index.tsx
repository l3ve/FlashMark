import styled from "@emotion/styled";
import { Global, css } from "@emotion/react";
import React from "react";
import { createRoot } from "react-dom/client";
import { AddBtn } from "./add";
import { List } from "./list";

let App = styled.div`
  position: relative;
  width: 100%;
  height: 100%;
`;

function Box() {
  return (
    <>
      <Global
        styles={css`
          html,
          body {
            border-radius: 6px;
            font-family: "PingFang SC", "Hiragino Sans GB", "Heiti SC",
              "Microsoft YaHei", "WenQuanYi Micro Hei";
            background-color: whitesmoke;
            overflow: hidden;
          }
          * {
            padding: 0;
            margin: 0;
            box-sizing: border-box;
          }
          #flash-mark {
            height: 500px;
          }
        `}
      />
      <App>
        <List></List>
        <AddBtn />
      </App>
    </>
  );
}

let body = document.getElementById("flash-mark");
if (!body) {
  body = document.createElement("div");
  body.id = "flash-mark";
  document.body.appendChild(body);
}

const root = createRoot(body);
root.render(<Box />);
