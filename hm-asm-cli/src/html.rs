use hm_asm_simulator::simulate::State;
use std::fmt::Write;

static TABLE_HEADER: &str = "
<table>
<thead>
  <tr>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">Schritt</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">clk</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">PC<br></th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">Addressbus</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">Datenbus</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">IR</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">DR</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">A</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">SR</th>
    <th style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">Bei Befehlen, die aus dem Speicher laden<br>bzw. in Speicher schreiben (LDA n, ADD n, STA n)</th>
  </tr>
</thead>\n";

pub fn html_state_table(states: Vec<State>) -> String {
    let mut result = String::from(TABLE_HEADER);
    result.push_str("<tbody>\n");
    for state in states.iter() {
        write!(result, "{}", state).unwrap();
    }

    result.push_str("</tbody>\n</table>");

    result
}
