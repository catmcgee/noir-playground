import React from "react";
import "src/app/globals.css";

type InstructionsProps = {
  instructions: string;
};

const Instructions: React.FC<InstructionsProps> = ({ instructions }) => {
  return (
    <div
      style={{ overflowY: "auto", maxHeight: "100%", wordBreak: "break-word" }}
    >
      <div
        dangerouslySetInnerHTML={{
          __html: instructions || "Select a challenge from the dropdown",
        }}
      />
    </div>
  );
};

export default Instructions;
