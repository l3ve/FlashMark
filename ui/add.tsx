import React, { useEffect } from "react";

function AddBtn() {
  useEffect(() => {
    console.log("333");
  }, []);
  return <>Flash Mark!</>;
}

export { AddBtn };
