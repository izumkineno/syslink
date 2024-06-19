import { dialog, invoke } from '@tauri-apps/api'
import { defineStore } from 'pinia'
import { reactive } from 'vue'

export interface linkFile {
    source: string
    target: string
}

export enum linkType {
    File,
    Files,
    Dir,
    Dirs,
    DirAllFiles,
    DirSurface,
    Hard,
    Hards,
    HardsDirAllFiles
}

export interface link {
    id: string,
    name: string,
    lname: string,
    diable_name: boolean,
    source: string[],
    is_dir: boolean,
    is_multi: boolean,
    target: string,
    type: linkType,
    time: string,
    children: Array<linkFile>
}

export interface symlinkFile {
    id: string,
    source: string,
    target: string,
    type_: string | number,
}

export interface symlinkData {
    files: symlinkFile[],
    files_id: string,
    id: string,
    name: string,
    source: string,
    target: string,
    time: string,
    type_: string | number,
}

export const symlinkStore = defineStore('symlink_table', () => {
    
    const link: link = reactive({
        id: "",
        name: "",
        lname: "",
        diable_name: false,
        source: [],
        is_dir: false,
        is_multi: false,
        target: "",
        type: linkType.File,
        time: "",
        children: []
    })

    const disable_name = (use = true) => {
        link.source = []
        link.diable_name = use
    }

    const set_dir_multi = (is_dir: boolean, is_multi: boolean) => {
        link.is_dir = is_dir
        link.is_multi = is_multi
    }

    const linkTypeSel: {
        label: string
        value: linkType
        click: () => void
    }[] = [
        {
            label: "单文件",
            value: linkType.File,
            click: () => {
                disable_name(false)
                set_dir_multi(false, false)
            }
        },
        {
            label: "多文件",
            value: linkType.Files,
            click: () => {
                disable_name()
                set_dir_multi(false, true)
            }
        },
        {
            label: "单目录",
            value: linkType.Dir,
            click: () => {
                disable_name(false)
                set_dir_multi(true, false)
            }
        },
        {
            label: "多目录",
            value: linkType.Dirs,
            click: () => {
                disable_name()
                set_dir_multi(true, true)
            }
        },
        {
            label: "所有文件",
            value: linkType.DirAllFiles,
            click: () => {
                disable_name()
                set_dir_multi(true, false)
            }
        },
        {
            label: "文件夹第一层所有",
            value: linkType.DirSurface,
            click: () => {
                disable_name()
                set_dir_multi(true, false)
            }
        },
        {
            label: "单文件(硬)",
            value: linkType.Hard,
            click: () => {
                disable_name(false)
                set_dir_multi(false, false)
            }
        },
        {
            label: "多文件(硬)",
            value: linkType.Hards,
            click: () => {
                disable_name()
                set_dir_multi(false, true)
            }
        },
        {
            label: "所有文件(硬)",
            value: linkType.HardsDirAllFiles,
            click: () => {
                disable_name()
                set_dir_multi(true, false)
            }
        }
    ]

    const open_source = () => {
        dialog.open({
            directory: link.is_dir,
            multiple: link.is_multi,
        }).then(res => {
            console.log(res);
            if(res != null) {
                link.source = (typeof res === 'string' ? [res] : res) as string[]
            }
        })
    }

    const open_target = () => {
        dialog.open({
            directory: true,
        }).then(res => {
            console.log(res);
            if(res != null) {
                link.target = res as string
            }
        })
    }

    const symlink_data= reactive<{
        data: Array<symlinkData>
    }>({
        data: []
    })

    // cmd：建立链接
    const symlink = () => {
        invoke("link", {
            source: typeof link.source === 'string' ? [link.source] : link.source,
            target: link.target,
            t: link.type,
            name: link.diable_name || link.name.trim() == "" ? undefined : link.name,
            lname: link.lname
        }).finally(() => {
            read_symlink_data()
        })
    }

    // cmd：刷新表格数据
    const read_symlink_data = () => {
        invoke("read_sled_from_db").then(res => {
            const v = res as Array<symlinkData>
            console.log(v);
            symlink_data.data = v
        })
    }

    // cmd：读取表格中文件数据
    const read_symlink_files_data = async (id: string, useAll: boolean) => {
        return await invoke("read_sled_files_from_db", {
            id,
            all: useAll
        })
    }

    // cmd：删除链接
    const remove_symlink = async (fss: Array<symlinkData>) => {
        return await invoke("remove_sled_from_db", {
            fss
        }).finally(() => {
            read_symlink_data()
        })
    }
    
    return {
        link,
        linkTypeSel,
        open_source,
        open_target,
        symlink,
        read_symlink_data,
        read_symlink_files_data,
        remove_symlink,
        symlink_data
    }
})