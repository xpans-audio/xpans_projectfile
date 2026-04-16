# xpans ProjectFile
A data format for the xpans Ecosystem combining necessary information to load 
audio and spatial data of scenes in addition to other descriptive information.

## Motivation
In the xpans Ecosystem, spatial audio scenes are a combination of audio and
spatial data, usually located in separate files. With a ProjectFile, users 
can avoid having to manually point applications to these separate files
in order to load a scene.

ProjectFiles also contain other identifying and descriptive information, such
as a title and optional names for audio sources.

## Example
Here's an example of how a ProjectFile would look in JSON:

```json
{
  "title": "My Project",
  "audio": "audio.wav",
  "spatial": "spatial.xsr",
  "source_names": {
    "0": "Main Vocal",
    "1": "Background Vocal 1",
    "2": "Background Vocal 2"
  }
}
```

## Paths
`audio` and `spatial` paths are relative to the ProjectFile's parent directory.
