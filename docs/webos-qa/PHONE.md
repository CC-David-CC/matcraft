# MatCraft on WebOS

The MatCraft launcher is in **Office**. Run a small matrix script, save it from
the script toolbar, and reopen it from **Files → Open**. The phone workspace
shows a touch editor and output pane. **Studio** opens the full MatCraft canvas.
A wide window defaults to Studio.

The hosted viewer tested here is
`7f2ef5f0e62b1db0ac50341b6e8c7243a6af722fb4f25716e236b4e4eaf5351f`.
The native MatCraft engine is pinned to
`9e0f03ba304b9fcd941bb63bf9d671762b8e3d59`.
WebOS builds the app from the pinned WASM package with the MatCraft host
wrapper in `assembly/viewer/`. The full viewer build accepts that verified
package via `--matcraft-site`.

## Phone workflow

1. In MatCraft, enter this two-line script and tap **Run script**.

   ```matlab
   A = [1 2; 3 4];
   disp(det(A));
   ```

   The result is `-2`. An incomplete matrix shows a syntax error. Correct it
   with `A = [2 1; 1 3];`, keep the second line, and run again for `5`.
2. Tap **Save as**. The suggested name is `Script.m`. Type a name without an
   extension and save it in My Files. WebOS adds `.m`.
3. Close MatCraft. In Files, select the new script and tap **Open**. The
   editable source returns, and **Run script** returns `5` again. Change the
   entries, **Save**, and run again for `-2`.

[Matrix result on Galaxy S10+](portrait-matrix.png) ·
[Save confirmation](portrait-saved.png) ·
[Editable script reopened from Files](portrait-reopened.png)

## Coverage

| Workflow | S10+ portrait | Desktop Chromium |
| --- | --- | --- |
| Write and run a matrix script | Pass | Pass |
| Read output and recover from malformed input | Pass | Pass |
| Save As with a default extension | Pass | Pass |
| Open the saved script through Files and save a later revision | Pass | Pass |
| Reopen a server-backed script from a fresh signed-in session | | Pass in isolated WebOS |
| Use MatCraft's own Open picker and cancel a discard | | Pass |
| Move an edited buffer between the touch editor and Studio | | Pass |
| Plot a line in Studio and observe the result in the Command Window | | Pass |
| Interactive 3D figure tools and graph export | | Open |
| Desktop simulator interop and advanced editor workflows | | Open |

The Galaxy S10+ used Chrome 158.0.8127.92 on the hosted guest release. The Moto
G 5G repeated the core workflow in landscape in an isolated install using
Chrome 158.0.8127.42. The emulated touch tablet repeated that workflow at
820 × 480 CSS pixels in Chromium. Desktop Chromium 158.0.8127.36 used guest
storage on ksu.codes and a disposable signed-in account in the isolated
installation. The guest files live in the current browser profile.

The release record gives the rollback to viewer `6676a3842adb`.
[WebOS release history](https://github.com/CC-David-CC/WebOS/blob/webos/matcraft-qualification/deploy/ACTIVE-RELEASE.md)
