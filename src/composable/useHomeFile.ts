import { cmds } from "@/plugins";

export default function () {
  return {
    writeBinaryFile2Home: async (filename: string, file: ArrayBuffer) => {
      const { home_dir } = await cmds.app_info();
      return cmds.writeBinaryFile2Home(home_dir + `/${filename}`, file);
    },
    readBinaryFile2Home: async (filename: string) => {
      const { home_dir } = await cmds.app_info();
      return cmds.readBinaryFile2Home(home_dir + `/${filename}`);
    },
  };
}
