use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use wasm_bindgen::__rt::std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub(crate) struct LogicalExpression {
    link: ComponentLink<Self>,
    props: LogicalExpressionProperties,
}

pub enum LogicalExpressionMessage {
    Ignore,
    PlusCol,
    MinusCol,
    FlipState(usize),
    UpdateExpression(HashSet<BitList>),
}

#[derive(Clone, Properties)]
pub struct LogicalExpressionProperties {
    num_args: usize,
    bit_state: HashSet<usize>,
    expression: HashSet<BitList>,
}

impl Default for LogicalExpressionProperties {
    fn default() -> Self {
        Self {
            num_args: 1,
            bit_state: HashSet::new(),
            expression: HashSet::new(),
        }
    }
}

impl LogicalExpression {
    fn calc_expression(&self) {
        let bit_length = self.props.num_args;
        let mut bit_list = HashSet::new();
        let bit_state = self.props.bit_state.iter().map(|i| {
            (0..bit_length).map(|j| if ((i >> j) & 1) == 1 { Bit::One } else { Bit::Zero }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let mut current_list = vec![HashSet::new(); bit_length + 1];
        let mut new_list = vec![HashSet::new(); bit_length + 1];
        for bit in bit_state {
            let one_count: usize = bit.iter().map(|b| if b.clone() == Bit::One { 1 } else { 0 }).sum();
            current_list[one_count].insert(BitList(bit.clone()));
        }
        loop {
            let mut used = HashSet::new();
            for i in 0..current_list.len() - 1 {
                for bit1 in &current_list[i] {
                    for bit2 in &current_list[i + 1] {
                        assert_eq!(bit1.0.len(), bit2.0.len());
                        let mut new_bit = BitList(Vec::new());
                        let mut diff_count = 0;
                        let mut one_count = 0;
                        for j in 0..bit1.0.len() {
                            if bit1.0[j] != bit2.0[j] {
                                diff_count += 1;
                                if diff_count > 1 { break; }
                                new_bit.0.push(Bit::DontCare);
                            } else {
                                if bit1.0[j] == Bit::One { one_count += 1; }
                                new_bit.0.push(bit1.0[j]);
                            }
                        }
                        if diff_count <= 1 {
                            used.insert(bit1);
                            used.insert(bit2);
                            new_list[one_count].insert(new_bit);
                        }
                    }
                }
            }
            for v in &current_list {
                for bit in v {
                    if !used.contains(bit) {
                        bit_list.insert(bit.clone());
                    }
                }
            }
            if used.len() == 0 { break; }
            current_list = new_list.clone();
            new_list = vec![HashSet::new(); bit_length + 1];
        }
        self.link.callback(|expression| LogicalExpressionMessage::UpdateExpression(expression)).emit(bit_list);
    }
}

impl Component for LogicalExpression {
    type Message = LogicalExpressionMessage;
    type Properties = LogicalExpressionProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::PlusCol => if self.props.num_args < 16 {
                self.props.num_args += 1;
                self.calc_expression();
                true
            } else { false },
            Self::Message::MinusCol => if self.props.num_args > 1 {
                self.props.num_args -= 1;
                self.calc_expression();
                true
            } else { false },
            Self::Message::FlipState(i) => {
                if self.props.bit_state.contains(&i) {
                    self.props.bit_state.remove(&i);
                } else {
                    self.props.bit_state.insert(i);
                }
                self.calc_expression();
                false
            }
            Self::Message::UpdateExpression(expression) => {
                self.props.expression = expression;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let head = (0..self.props.num_args).map(|i| html! {<th scope="col">{format!("x{}",i)}</th>});
        let body = (0..(1 << self.props.num_args)).map(|i| {
            let data = (0..self.props.num_args).map(|j| html! {<td>{(i>>j)&1}</td>});
            html! {<tr>{for data}<td><input type="checkbox" checked={self.props.bit_state.contains(&i)} onchange=self.link.callback(move|_|Self::Message::FlipState(i))/></td></tr>}
        });
        let expression = if self.props.expression.iter().any(|b| b.0.iter().any(|b| b != &Bit::DontCare)) {
            self.props.expression.iter().zip(0..self.props.expression.len()).map(|bits| {
                let (bits, index) = bits;
                let bits = bits.0.iter().zip(0..bits.0.len()).map(|b| match b.0 {
                    Bit::One => html! {<span style="text-decoration: none;">{format!("{}x{}",if b.1!=0{" "}else{""},b.1)}</span>},
                    Bit::Zero => html! {<span style="text-decoration: overline;">{format!("{}x{}",if b.1!=0{" "}else{""},b.1)}</span>},
                    Bit::DontCare => html! {{""}}
                });
                html! {<span>{if index!=0{"|"}else{""}}{for bits}</span>}
            }).collect::<Vec<_>>()
        } else { vec![html! {<span>{if self.props.expression.len()>0{1}else{0}}</span>}] };

        html! {
            <div class="logical_expression">
                <div>
                    <button class="btn btn-secondary" onclick=self.link.callback(|_|Self::Message::MinusCol)>{"-1"}</button>
                    <button class="btn btn-secondary" onclick=self.link.callback(|_|Self::Message::PlusCol)>{"+1"}</button>
                </div>
                <p class="lead my-sm-3">
                    {"y="}{for expression}
                </p>
                <table class="table">
                    <thead>
                        <tr>
                            {for head}
                            <th scope="col"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {for body}
                    </tbody>
                </table>
            </div>
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Bit {
    One,
    Zero,
    DontCare,
}

#[derive(Clone)]
pub struct BitList(Vec<Bit>);

impl PartialEq for BitList {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() { return false; }
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] { return false; }
        }
        true
    }
}

impl Eq for BitList {}

impl Hash for BitList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for bit in &self.0 {
            state.write_u8(
                match bit {
                    Bit::Zero => 0,
                    Bit::One => 1,
                    Bit::DontCare => 2,
                });
        }
    }
}