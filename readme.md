### features

- [x]  基础软/硬映射

- [x]  保存映射记录

- [x]  解除映射

- [ ]  链接失败时弹出确认，并处理失败情况



### 建立符号链接可能出现的问题

- **权限错误（Permission Denied）**：
  
  - 发生原因：用户没有在目标目录或文件上创建符号链接的权限。
  - 解决方法：使用具有适当权限的用户（例如，使用 `sudo` 提升权限）来创建符号链接。

- **目标不存在（No Such File or Directory）**：
  
  - 发生原因：指定的目标文件或目录不存在。
  - 解决方法：检查目标路径是否正确，确保目标文件或目录存在。

- **文件系统不支持（Operation Not Permitted）**：
  
  - 发生原因：文件系统不支持符号链接（例如，一些FAT文件系统不支持符号链接）。
  - 解决方法：确保在支持符号链接的文件系统上操作（例如，ext4、NTFS等）。

- **符号链接已存在（File Exists）**：
  
  - 发生原因：要创建的符号链接已经存在。
  - 解决方法：可以选择删除现有的符号链接或使用不同的名称创建新的符号链接。

- **文件系统已满（No Space Left on Device）**：
  
  - 发生原因：文件系统没有足够的空间来创建新的符号链接。
  - 解决方法：清理一些文件以释放空间，或者使用具有更多可用空间的文件系统。

- **跨设备链接（Invalid Cross-device Link）**：
  
  - 发生原因：试图在不同的文件系统之间创建硬链接（不是符号链接），但硬链接不支持跨文件系统操作。
  - 解决方法：使用符号链接而不是硬链接，符号链接可以跨文件系统创建。

- **无效的符号链接路径（Invalid Argument）**：
  
  - 发生原因：提供的路径格式不正确或包含非法字符。
  - 解决方法：检查并修正符号链接的路径，确保路径格式正确。
