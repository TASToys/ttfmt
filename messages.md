# Messages


## FidoFmt

The **F.I.D.O** (Formated internal data object) represents the comman information gathered by chatserv that is sent to state processor or botserv to be
to be processed.

| Field           |Type         | Description|
| ---------       |-----        |--------- |
| plugin_id       | **Integer** | The plugin id of the plugin this message corresponds to |
| plugin_type     | **Integer** | The plugin type signifies wether the messages need to be processed by state processor or if it can be passed directly to botserv. **1** means the messages needs to be processed by state processor. **-1** means the message be passed to botserv without processing. A message should never return **0** |
| source_platform | **String**  | The platform the message is sent from i.e twitch, YouTube, HitBox |
| source_channel  | **String**  | The channel the message is sent from i.e [dwangoAC](https://www.twitch.tv/dwangoac) |
| timestamp       | **String**  | The time the message was sent in the unix timestamp format |
| source_user     | **String**  | The user who issued the command that caused the fido to be generated |
| data            | **Object**  | A data object that is sent with the relevant information required upstream as well as platform meta data |

### Example message
```json
{
    "plugin_id":123,
    "plugin_type":-1,
    "source_platform":"twitch",
    "source_channel":"dwangoAC",
    "timestamp":"1522896164",
    "source_user":"Jeff",
    "data": {
        ....
    }
}
```

## ChatFmt
The ChatFmt is a data format that normalizes chat from many different sources (i.e Twitch, YouTube, HitBox) into a common data format to be processed for command information to be used for the **F.I.D.O.** and to be logged by history serv this should not be used by any other address.

| Field           |Type             | Description|
| ---------       |-----            |---------   |
| platform        | **String**      | The platform the chat message came from (i.e Twitch, YouTube, HitBox) |
| channel         | **String**      | The channel the server came from (i.e dwangoAC) |
| timestamp       | **String**      | The timestamp of when the chat message was sent sent |
| sender          | **String**      | The one who sent the chat message |
| message         | **String**      | The chat message |
| split_msg       | **String Array**| The chat message split into individual words|
| platform_meta   | **Object**      | An object containing platfrom metadata (i.e isMod, isSteamer, badges, etc...)

### Example message

```json
{
    "platform":"twitch",
    "channel":"dwangoAC",
    "timestamp":"1522899575",
    "sender":"Jeff",
    "message":"Lets sit around the campfire and sing the campfire song the C A M P F I R E song",
    "split_msg":["Lets", "sit", "around", "the", "campfire", "and", "sing", "the", "campfire",
    "song", "the", "C", "A", "M", "P", "F", "I", "R", "E", "song"],
    "platform_meta": {
        ....
    }

}
```
