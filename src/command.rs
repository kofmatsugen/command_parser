use crate::types::{build_command, button::Key, input::CommandKey};

#[derive(Debug)]
pub struct Command {
    keys: Vec<CommandKey>,
}

impl Command {
    pub fn build(command: &str) -> Result<Self, failure::Error> {
        Ok(build_command(command)?)
    }

    pub fn new(keys: Vec<CommandKey>) -> Self {
        Command { keys }
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &CommandKey> {
        self.keys.iter()
    }

    // キー入力バッファからコマンドが成立したか判定
    pub fn judge_inputs(
        &self,
        inputs: impl DoubleEndedIterator<Item = Key>,
        default_buffer: u32,
        default_hold: u32,
    ) -> bool {
        // 逆順に判定していく
        // 消費しながら使い回すので参照取得
        let mut inputs_rev = inputs.rev();
        let inputs_rev = inputs_rev.by_ref();
        self.keys().rev().all(|key| match key {
            &CommandKey::Push { key, buffer_frame } => {
                log::trace!("push: Key = {:?}, buffer = {:?}", key, buffer_frame);
                let buffer_frame = buffer_frame.unwrap_or(default_buffer);
                // ボタンを押したときは直前がそのボタンじゃなかったとき
                // 最後の入力を見つけたあと，その後のカウント数と合わせてバッファフレーム内に収まるか
                let position = inputs_rev
                    .position(|input| input.contains(key))
                    .map(|p| p as u32);

                let first_input_count =
                    inputs_rev.take_while(|input| input.contains(key)).count() as u32;

                // 最後の入力がバッファフレーム以内ならOK
                position
                    .map(|p| p + first_input_count < buffer_frame)
                    .unwrap_or(false)
            }
            &CommandKey::Release { key, buffer_frame } => {
                log::trace!("release: Key = {:?}, buffer = {:?}", key, buffer_frame);
                let buffer_frame = buffer_frame.unwrap_or(default_buffer);
                // ボタン離しは最後に入力があったもののあとなので，最後の入力位置を探す
                let position = inputs_rev
                    .position(|input| input.contains(key))
                    .map(|p| p as u32);
                // 見つけたのは最後の入力なので，その直後からバッファフレーム内ならOK
                position
                    .map(|p| p > 0 && p < buffer_frame + 1)
                    .unwrap_or(false)
            }
            &CommandKey::Hold {
                key,
                buffer_frame,
                hold_frame,
            } => {
                log::trace!(
                    "hold: Key = {:?}, hold = {:?}, buffer = {:?}",
                    key,
                    hold_frame,
                    buffer_frame
                );
                let buffer_frame = buffer_frame.unwrap_or(default_buffer);
                let hold_frame = hold_frame.unwrap_or(default_hold);
                let position = inputs_rev
                    .position(|input| input.contains(key))
                    .map(|p| p as u32);
                // 最後の入力から離しの1F + バッファ分まではタメとして許容
                let buffer_ok = position.map(|p| p < buffer_frame + 1).unwrap_or(false);

                // 以降該当の入力が hold_frame 分あればOK
                // 直前に最後1F分を取得しているので じっさいは hold_frame - 1
                let hold_count = inputs_rev.take_while(|input| input.contains(key)).count() as u32;
                let hold_ok = hold_count >= hold_frame - 1;

                // バッファ内かつタメ時間をクリアしていればOK
                buffer_ok && hold_ok
            }
            &CommandKey::On { key } => {
                // 最後の入力が必要な入力ならOK
                inputs_rev
                    .next()
                    .map(|input| input.contains(key))
                    .unwrap_or(false)
            }
            &CommandKey::Off { key } => {
                // 最後の入力が必要な入力を含んでいなければOK
                inputs_rev
                    .next()
                    .map(|input| input.contains(key) == false)
                    .unwrap_or(true)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const COMMAND: &'static str = "h4(60)[10] > p6[10] > pC[10]";
    #[test]
    fn judge_ok() {
        let command = build_command(COMMAND).unwrap();
        let inputs = (0..10)
            .map(|_| Key::empty())
            .chain((0..120).map(|_| Key::BACKWARD))
            .chain((0..9).map(|_| Key::empty()))
            .chain((0..10).map(|_| Key::FORWARD))
            .chain((0..1).map(|_| Key::C))
            .chain((0..9).map(|_| Key::empty()));
        assert_eq!(command.judge_inputs(inputs, 10, 10,), true);
    }

    #[test]
    fn judge_fail_hold() {
        let command = build_command(COMMAND).unwrap();
        let inputs = (0..10)
            .map(|_| Key::empty())
            .chain((0..59).map(|_| Key::BACKWARD))
            .chain((0..1).map(|_| Key::FORWARD))
            .chain((0..10).map(|_| Key::C));
        assert_eq!(command.judge_inputs(inputs, 10, 10,), false);
    }
}
