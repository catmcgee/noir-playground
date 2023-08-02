import React, { useEffect } from "react";
import ReactAce from "react-ace";

import "ace-builds/src-noconflict/theme-github";
import "ace-builds/src-noconflict/mode-rust";
import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/ext-beautify";

type CodeEditorProps = {
  code: string;
  setCode: (code: string) => void;
};

const CodeEditor: React.FC<CodeEditorProps> = ({ code, setCode }) => {
  return (
    <div>
      <h2 className="font-bold text-lg mb-2">Noir Code</h2>
      <ReactAce
        mode="rust"
        theme="github"
        value={code}
        onChange={setCode}
        name="codeEditor"
        editorProps={{ $blockScrolling: true }}
        fontSize={14}
        showPrintMargin={true}
        showGutter={true}
        highlightActiveLine={true}
        setOptions={{
          enableBasicAutocompletion: true,
          enableLiveAutocompletion: false,
          enableSnippets: false,
          showLineNumbers: true,
          tabSize: 2,
          minLines: 15,
          maxLines: Infinity,
        }}
        className="w-full rounded-lg"
        style={{ height: "calc(100% - 2rem)", width: "100%" }}
      />
    </div>
  );
};

export default CodeEditor;
